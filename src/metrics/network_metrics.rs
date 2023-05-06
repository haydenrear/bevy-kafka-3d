use std::marker::PhantomData;
use bevy::prelude::{Commands, Component, Entity, Query, ResMut, Resource};
use bevy::utils::HashMap;
use crate::network::Node;

#[derive(Component, Clone, Debug, Default)]
pub struct Metric <T>
where T: Component {
    metric_name: &'static str,
    metric_value: f32,
    historical: HistoricalData,
    metric_type: MetricType<T>,
    pub(crate) dirty: bool
}

#[derive(Clone, Debug)]
pub enum MetricType<T>
where T: Component
{
    WeightVariance(PhantomData<T>),
    Concavity(PhantomData<T>),
    Loss(PhantomData<T>)
}

impl <T> Default for MetricType<T> where T: Component
{
    fn default() -> Self {
        Self::WeightVariance(PhantomData::<T>::default())
    }
}


#[derive(Default, Component, Clone, Debug)]
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
