use std::default::{default, Default};
use bevy::prelude::{Component, Entity, Resource};
use bevy::utils::HashMap;


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


pub struct LayerMetrics {
    pub(crate) dirty: bool,
    pub(crate) metrics: HashMap<Entity, Metric>
}

impl LayerMetrics {
    pub(crate) fn new(metrics: HashMap<Entity, Metric>) -> Self {
        Self {
            dirty: true, metrics
        }
    }
}

#[derive(Default, Resource)]
pub struct MetricState {
    pub(crate) metrics: HashMap<Entity, LayerMetrics>
}

#[derive(Default, Resource)]
pub struct MetricsSubscription {
}

#[derive(Default, Resource)]
pub struct MetricsMetadataSubscription {
}

