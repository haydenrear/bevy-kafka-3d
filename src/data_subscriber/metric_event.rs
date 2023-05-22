use serde::{Deserialize, Serialize};
use bevy::prelude::{Component, Entity};
use std::collections::{HashMap, LinkedList};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::{Arc, mpsc, Mutex};
use bevy::prelude::{Condition, error, Events, EventWriter, Res, ResMut, Resource, World};
use bevy::utils::petgraph::visit::Walker;
use crate::metrics::network_metrics::Metric;
use crate::network::{Layer, MetricChildNodes, Network, Node};

pub trait NetworkEvent: for<'a> Deserialize<'a> + Send + Sync {
    fn topic_matcher() -> &'static str;
}

pub trait NetworkMetricsServiceEvent<C>: for<'a> Deserialize<'a> + Send + Sync + NetworkEvent
where C: Component
{
    fn metric_name(&self) -> &str;
    fn metric_indices(&self) -> HashMap<MetricComponentType, Vec<String>>;
    fn get_shape(&self) -> &Vec<usize>;
    fn get_data(&self) -> Vec<f32>;
    fn get_included(&self) -> &Vec<u32>;
    fn get_columns(&self) -> Option<HashMap<String, usize>>;
}

#[derive(Default, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub enum MetricComponentType {
    Layer,
    Node,
    #[default]
    Network
}

macro_rules! network_events {
    ($($event_type:ident, $event_component:ty, $event_lit:literal),*) => {
        $(

            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct $event_type {
                pub(crate) shape: Vec<usize>,
                pub(crate) data: Mutex<Option<Vec<f32>>>,
                pub(crate) metric_name: String,
                pub(crate) included: Vec<u32>,
                pub(crate) columns: Option<HashMap<String, usize>>,
                pub(crate) metric_indices: Option<HashMap<MetricComponentType, Vec<String>>>
            }

            impl NetworkEvent for $event_type {
                fn topic_matcher() -> &'static str {
                    $event_lit
                }
            }

            impl NetworkMetricsServiceEvent<$event_component> for $event_type {
                fn metric_name<'a>(&'a self) -> &'a str {
                    self.metric_name.as_str()
                }
                fn metric_indices(&self) -> HashMap<MetricComponentType, Vec<String>> {
                    self.metric_indices.clone()
                        .or(Some(HashMap::new()))
                        .unwrap()
                }
                fn get_included(&self) -> &Vec<u32> {
                    &self.included
                }
                fn get_shape(&self) -> &Vec<usize> {
                    &self.shape
                }
                fn get_data(&self) -> Vec<f32> {
                    let mut return_val = None;
                    let _ = self.data.lock()
                        .as_mut()
                        .map(|inner| {
                            std::mem::replace(&mut return_val, inner.take())
                        })
                        .or_else(|err| {
                            error!("Error replacing data: {:?}.", err);
                            Err(err)
                        });
                    return_val.or(Some(vec![]))
                        .unwrap()
                }
                fn get_columns(&self) -> Option<HashMap<String, usize>> {
                    self.columns.clone()
                }
            }
        )*
    }
}

network_events!(
    NodeMetricEvent, Node, "node_metric_*",
    LayerMetricEvent, Layer, "layer_metric_*",
    NetworkMetricEvent, Network, "network_metric_*",
    NodeChildrenMetricEvent, MetricChildNodes, "node_as_children_metric_*",
    MetricMessage, Metric<Network>, "metric_*"
);

#[derive(Resource, Debug, Default)]
pub struct MetricsState {
    pub(crate) entities: HashMap<String, (Entity, u64)>,
}

impl MetricsState {
    pub(crate) fn get_entity(&self, name: &str) -> Option<(Entity, u64)> {
        self.entities.get(name)
            .map(|(entity, timestep)| (*entity, *timestep))
    }

    pub(crate) fn increment_entity(&mut self, name: &str) {
        self.entities.get_mut(name)
            .as_mut()
            .map(|entity_state| {
                entity_state.1 += 1;
            });
    }
}
