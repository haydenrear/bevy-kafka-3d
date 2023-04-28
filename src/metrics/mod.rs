use std::default::{default, Default};
use bevy::prelude::{BuildChildren, Children, Commands, Component, Entity, info, Query, Res, ResMut, Resource};
use bevy::utils::HashMap;
use bevy_prototype_lyon::prelude::tess::geom::Transform;
use crate::network::Node;


#[derive(Component, Clone)]
pub struct Metric {
    metric_name: &'static str,
    metric_value: f32,
    historical: HistoricalData,
    pub(crate) dirty: bool
}

impl Default for Metric {
    fn default() -> Self {
        Self {
            dirty: true,
            metric_name: "",
            metric_value: f32::default(),
            historical: HistoricalData::default()
        }
    }
}

#[derive(Default, Component, Clone)]
pub(crate) struct HistoricalData {
    data: Vec<f32>,
    write_index: usize,
    size: usize,
}

impl HistoricalData {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            data: vec![0.0; size],
            write_index: 0,
            size,
        }
    }

    pub(crate) fn push(&mut self, value: f32) {
        self.data[self.write_index] = value;
        self.write_index = (self.write_index + 1) % self.size;
    }

    pub(crate) fn get(&self, index: usize) -> Option<f32> {
        if index >= self.size {
            return None;
        }

        let read_index = (self.write_index + self.size - index - 1) % self.size;
        Some(self.data[read_index])
    }
}

/// Each metric can be associated with a group of nodes. So the metric can be the parent, and the children
/// of the network will then be the nodes for which it is associated, or the layer. This allows for drawing
/// the metric relative to the nodes, highlighting multiple associated nodes in accordance with the metrics
#[derive(Default, Resource)]
pub struct MetricState {
    pub(crate) metrics: HashMap<Entity, Metric>
}

#[derive(Resource)]
pub struct MetricsSubscription {
    did_update: bool
}

impl Default for MetricsSubscription {
    fn default() -> Self {
       Self {
           did_update: false
       }
    }
}

#[derive(Default, Resource)]
pub struct MetricsMetadataSubscription {
}

#[derive(Default, Component)]
pub struct MetricChildNodes {
    nodes: Vec<Entity>
}

pub(crate) fn update_metrics(
    mut commands: Commands,
    mut metrics_resource: ResMut<MetricsSubscription>,
    nodes: Query<(Entity, &mut Node)>,
) {
    for node in nodes.iter() {
        if !metrics_resource.did_update {
            for i in 0..10 {
                commands.spawn(Metric::default())
                    .insert(MetricChildNodes {
                        nodes: vec![node.0],
                    });
            }
        }
        metrics_resource.did_update = true;
    }
}

/// Each metric has it's children
pub(crate) fn publish_metrics(
    metrics: Query<(&Metric, &MetricChildNodes)>,
    child_nodes: Query<&Node>
) {
    for metric in metrics.iter() {
        if child_nodes.get(metric.1.nodes.get(0).unwrap().clone()).is_ok() {
            // info!("Successfully added MetricChildNodes.");
        }
    }
}