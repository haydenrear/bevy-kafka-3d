use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Mutex;
use bevy::log::info;
use bevy::prelude::{EventWriter, ResMut, Resource};
use crate::data_subscriber::metric_event::{NetworkEvent, NetworkMetricEvent, NodeMetricEvent};

#[derive(Resource, Default)]
pub struct TestEventGeneratingResource {
    num_iterations: usize,
    data_dim: usize
}

impl TestEventGeneratingResource {
    pub(crate) fn new(data_dim: usize) -> Self {
        Self {
            num_iterations: 0,
            data_dim
        }
    }
}

impl TestEventGeneratingResource {
    fn create_fake_network_event(&self, i: usize) -> NodeMetricEvent {
        let mut columns = HashMap::new();
        let num = self.data_dim;
        for x in 0..num {
            columns.insert(x.to_string(), x);
        }
        let metric = NodeMetricEvent {
            shape: vec![num],
            data: Mutex::new(Some(vec![i as f32; num])),
            metric_name: "metric".to_string(),
            included: vec![],
            columns: Some(columns),
            metric_indices: None,
        };
        metric
    }
}

pub(crate) fn write_fake_metric_network_events(
    mut event_writer: EventWriter<NodeMetricEvent>,
    mut event_resource: ResMut<TestEventGeneratingResource>
) {
    info!("Sending fake network event.");
    event_writer.send(event_resource.create_fake_network_event(event_resource.num_iterations));
    event_resource.num_iterations += 1;
}


