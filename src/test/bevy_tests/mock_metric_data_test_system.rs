use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Mutex;
use bevy::prelude::{EventWriter, Resource};
use crate::data_subscriber::metric_event::{NetworkEvent, NetworkMetricEvent, NodeMetricEvent};

#[derive(Resource)]
pub struct TestEventGeneratingResource {
}

impl TestEventGeneratingResource {
    fn create_fake_network_event() -> NodeMetricEvent {
        let metric = NodeMetricEvent {
            shape: vec![3],
            data: Mutex::new(Some(vec![0.0, 1.0, 2.0, 3.0])),
            metric_name: "metric".to_string(),
            included: vec![],
            columns: Some(HashMap::from([
                ("first".to_string(), 0),
                ("second".to_string(), 1),
                ("third".to_string(), 2),
                ("fourth".to_string(), 2)
            ])),
        };
        metric
    }
}

pub(crate) fn write_fake_metric_network_events(
    mut event_writer: EventWriter<NodeMetricEvent>,
) {
    event_writer.send(TestEventGeneratingResource::create_fake_network_event());
}


