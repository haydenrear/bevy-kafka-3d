use std::marker::PhantomData;
use bevy::prelude::{Commands, Component, Entity, error, Query, ResMut, Resource};
use bevy::utils::HashMap;
use ndarray::{Array, Array1, Array2, ArrayBase, ArrayD, ArrayView, ArrayView1, Axis, Dim, Ix, Ix0, Ix1, Ix2, IxDyn, OwnedRepr, s, Shape, ShapeBuilder, Slice, SliceArg, SliceInfoElem, ViewRepr};
use serde::{Deserialize, Deserializer};
use serde::de::EnumAccess;
use crate::menu::Menu;
use crate::network::{ Node};

#[derive(Component, Clone, Debug, Default)]
pub struct Metric <T>
where T: Component {
    pub(crate) historical: HistoricalData,
    pub(crate) metric_type: MetricType<T>
}

impl <T> Metric<T> where T: Component {
    pub(crate) fn new(
        size: Vec<usize>,
        metric_type: MetricType<T>,
        labels: HashMap<String, usize>
    )  -> Metric<T> {
        Self {
            historical: HistoricalData::new(size, labels),
            metric_type
        }
    }
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
    pub(crate) timestep: HashMap<u64, (usize, usize)>,
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
            timestep: HashMap::new(),
            index_to_timestep: Default::default(),
        }
    }

    pub fn retrieve_values_inner(&self, column_name: &str, timestamp: u64) -> Option<ArrayBase<OwnedRepr<f32>, Ix1>>{
        self.labels.get(column_name)
            .map(|col| self.timestep.get(&timestamp).map(|t| (col, t)))
            .flatten()
            .map(|(label, time)| {
                self.data.index_axis(Axis(0), time.1)
                    .into_dimensionality::<Ix2>()
                    .or_else(|e| {
                        error!("Could not make dimension 2 array: {:?}", e);
                        Err(e)
                    })
                    .ok()
                    .map(|d| (d, *label))
            })
            .flatten()
            .map(|(all_data, label)| {
                    all_data.select(Axis(0), &[label])
                        .remove_axis(Axis(0))
            })
    }

    pub(crate) fn retrieve_values(&self, column_name: &str, timestamp: u64)
        -> (Option<ArrayBase<OwnedRepr<f32>, Ix1>>, Option<ArrayBase<OwnedRepr<f32>, Ix1>>) {
        let prev = self.get_prev_timestamp(timestamp);
        let prev = self.retrieve_values_inner(column_name, prev);
        let mut next = self.retrieve_values_inner(column_name, timestamp);
        if next.is_none() {
            next = prev.clone() ;
        }
        (prev, next)
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

        self.write_index += 1;

        if self.prev_write_index != 1 {
            self.prev_write_index += 1;
        }
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


#[derive(Default, Component, Clone, Debug)]
pub struct MetricChildNodes {
    nodes: Vec<Entity>
}

pub(crate) fn update_metrics(
    mut commands: Commands,
    // mut metrics_resource: ResMut<MetricsSubscription>,
    nodes: Query<(Entity, &mut Node)>,
) {
    // for node in nodes.iter() {
    //     if !metrics_resource.did_update {
    //         for i in 0..10 {
    //             commands.spawn(Metric::default())
    //                 .insert(MetricChildNodes {
    //                     nodes: vec![node.0],
    //                 });
    //         }
    //     }
    //     metrics_resource.did_update = true;
    // }
}

/// Each metric has it's children
pub(crate) fn publish_metrics(
    // metrics: Query<(&Metric, &MetricChildNodes)>,
    child_nodes: Query<&Node>
) {
    // for metric in metrics.iter() {
    //     if child_nodes.get(metric.1.nodes.get(0).unwrap().clone()).is_ok() {
            // info!("Successfully added MetricChildNodes.");
        // }
    // }
}
