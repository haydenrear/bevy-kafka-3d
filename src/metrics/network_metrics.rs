use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;
use bevy::log::info;
use bevy::prelude::{Commands, Component, Entity, error, Query, ResMut, Resource};
use ndarray::{Array, Array1, Array2, ArrayBase, ArrayD, ArrayView, ArrayView1, Axis, Dim, Ix, Ix0, Ix1, Ix2, IxDyn, OwnedRepr, s, Shape, ShapeBuilder, Slice, SliceArg, SliceInfoElem, ViewRepr};
use serde::{Deserialize, Deserializer};
use serde::de::EnumAccess;
use crate::data_subscriber::metric_event::MetricComponentType;
use crate::graph::{GraphDim, GraphDimType, GridAxis};
use crate::menu::Menu;
use crate::network::{Layer, Network, Node};

#[derive(Component, Clone, Debug, Default)]
pub struct Metric <T>
where T: Component {
    pub(crate) historical: HistoricalData,
    pub(crate) metric_type: MetricType<T>,
    pub(crate) metric_indices: HashMap<MetricComponentType, Vec<String>>
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MetricType<T>
    where T: Component
{
    WeightVariance(PhantomData<T>),
    Concavity(PhantomData<T>),
    Loss(PhantomData<T>)
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize)]
pub enum MetricTypeMatcher {
    WeightVariance,
    Concavity,
    Loss
}

impl <T> Metric<T> where T: Component {
    pub(crate) fn new(
        size: Vec<usize>,
        metric_type: MetricType<T>,
        labels: HashMap<String, usize>,
        metric_indices: HashMap<MetricComponentType, Vec<String>>
    )  -> Metric<T> {
        Self {
            historical: HistoricalData::new(size, labels),
            metric_type,
            metric_indices
        }
    }
}

impl MetricTypeMatcher {
    pub(crate) fn get_metric<T>(&self) -> MetricType<T>
    where T: Component
    {
        match self {
            MetricTypeMatcher::WeightVariance => {
                MetricType::WeightVariance(PhantomData::<T>::default())
            }
            MetricTypeMatcher::Concavity => {
                MetricType::Concavity((PhantomData::<T>::default()))
            }
            MetricTypeMatcher::Loss => {
                MetricType::Loss((PhantomData::<T>::default()))
            }
        }
    }
}

impl <T> Default for MetricType<T> where T: Component
{
    fn default() -> Self {
        Self::WeightVariance(PhantomData::<T>::default())
    }
}

#[derive(Default, Component, Clone, Debug)]
pub(crate) struct HistoricalData {
    pub(crate) data: ArrayD<f32>,
    pub(crate) labels: HashMap<String, usize>,
    pub(crate) timestep: BTreeMap<u64, (usize, usize)>,
    index_to_timestep: HashMap<usize, u64>,
    convergence: HashMap<String, HashMap<u64, f32>>,
    write_index: usize,
    prev_write_index: usize,
    size: Vec<usize>,
}

impl HistoricalData {

    pub(crate) fn new(size: Vec<usize>, labels: HashMap<String, usize>) -> Self {
        let mut size = size.clone();
        size.insert(0, 1);
        let mut convergence = HashMap::new();
        labels.iter().for_each(|(label, index)| {
            convergence.insert(label.clone(), HashMap::new());
        });
        Self {
            data: ArrayD::zeros(size.clone()),
            write_index: 1,
            size,
            labels,
            prev_write_index: 1,
            convergence,
            timestep: BTreeMap::new(),
            index_to_timestep: Default::default(),
        }
    }

    pub fn retrieve_values_inner(&self, column_name: &str, timestamp: u64) -> Option<(ArrayBase<OwnedRepr<f32>, Ix1>, ArrayBase<OwnedRepr<f32>, Ix1>)> {
        self.timestep.get(&timestamp)
            .map(|(prev, next)| {
                info!("Fetching timestep for {} with time {} with {} and {}", column_name, timestamp, prev, next);
                let historical_1d = self.retrieve_historical_1d(column_name);
                let prev = historical_1d.iter().map(|h| {
                    h[*prev]
                }).collect::<Vec<f32>>();
                let next = historical_1d.iter().map(|h| {
                    h[*next]
                }).collect::<Vec<f32>>();
                (Array::from_vec(prev), Array::from_vec(next))
            })

    }

    fn to_2d_arr(&self, label: &usize, time: &(usize, usize)) -> Option<(ArrayBase<OwnedRepr<f32>, Ix2>, usize)> {
        // let mut indexed = self.data.index_axis(Axis(0), time.1);
        let mut indexed = self.data.select(Axis(0), &[time.1]);
        info!("{:?} is the shape of the indexed axis, supposed to be along one time stamp: {}.", indexed.shape(), time.1);
        indexed.into_dimensionality::<Ix2>()
            .or_else(|e| {
                error!("Could not make dimension 2 array: {:?}. {:?} is the shape.", e, &self.data.shape());
                Err(e)
            })
            .ok()
            .map(|d| (d, *label))
    }

    fn is_1d(shape: &[usize]) -> bool {
        shape.len() <= 1 || shape[1] <= 1
    }

    pub(crate) fn retrieve_values(&self, column_name: &str, timestamp: u64)
                                  -> Option<(ArrayBase<OwnedRepr<f32>, Ix1>, ArrayBase<OwnedRepr<f32>, Ix1>)> {
        self.retrieve_values_inner(column_name, timestamp)
    }

    pub(crate) fn extend(&mut self, mut value: ArrayD<f32>, timestep: u64) {
        value.insert_axis_inplace(Axis(0));

        let _ = self.data.append(Axis(0), value.view())
            .or_else(|e| {
                error!("Error adding to historical data: {:?}", e);
                Err(e)
            });

        self.timestep.insert(timestep, (self.prev_write_index, self.write_index));
        self.index_to_timestep.insert(self.write_index, timestep);

        if self.write_index != 1 {
            self.prev_write_index += 1;
        }

        self.write_index += 1;

    }

    pub(crate) fn retrieve_historical(&self, column_name: &str) -> Option<ArrayD<f32>> {
        self.labels.get(column_name)
            .map(|column| {
                self.data.select(Axis(1), &[*column])
                    .remove_axis(Axis(1))
            })
    }

    pub(crate) fn retrieve_historical_1d(&self, column_name: &str) -> Vec<ArrayBase<OwnedRepr<f32>, Ix1>> {
        let h = self.retrieve_historical(column_name)
            .unwrap();

        info!("Retrieved historical for column: {} with shape: {:?}.", column_name, h.shape());

        let mut out = vec![];

        for i in h.columns() {
            let _ = i.into_dimensionality::<Ix1>()
                .map(|a| {
                    out.push(a.into_owned());
                });
        }

        out
    }

    pub(crate) fn get_timestamp(&self, step: usize) -> Option<u64> {
        self.index_to_timestep.get(&step)
            .cloned()
    }

    pub(crate) fn get_prev_timestamp(&self, timestamp: u64) -> u64 {
        self.timestep.get(&timestamp)
            .map(|(prev, next)| {
                let prev = self.index_to_timestep.get(prev)
                    .or(Some(&timestamp))
                    .unwrap();
                *prev
            })
            .or(Some(timestamp))
            .unwrap()
    }

    pub(crate) fn get(&self, index: &[usize]) -> Option<f32> {
        self.data.get(index)
            .cloned()
    }

}

