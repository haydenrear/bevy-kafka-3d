use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use kafka::client::fetch::Data;
use crate::data_subscriber::kafka_data_subscriber::{KafkaClientProvider, LayerMetricEvent, NodeChildrenMetricEvent, NetworkMetricEvent, NodeMetricEvent, consume_kafka_messages, write_events};

pub struct DataSubscriberPlugin;

// impl Plugin for DataSubscriberPlugin {
//     fn build(&self, app: &mut App) {
//         app.insert_resource(KafkaClientProvider::default())
//             .add_event::<NodeMetricEvent>()
//             .add_event::<LayerMetricEvent>()
//             .add_event::<NetworkMetricEvent>()
//             .add_event::<NodeChildrenMetricEvent>()
//             .add_startup_system(consume_kafka_messages::<NodeMetricEvent>)
//             .add_startup_system(consume_kafka_messages::<LayerMetricEvent>)
//             .add_startup_system(consume_kafka_messages::<NetworkMetricEvent>)
//             .add_startup_system(consume_kafka_messages::<NodeChildrenMetricEvent>)
//             .add_system(write_events::<NodeMetricEvent>)
//         ;
//     }
// }


macro_rules! network_plugin {
    ($($event_type:ident, $event_lit:literal),*) => {
        impl Plugin for DataSubscriberPlugin {
            fn build(&self, app: &mut App) {
                app.insert_resource(KafkaClientProvider::default())
                    $(
                        .add_event::<$event_type>()
                        .add_startup_system(consume_kafka_messages::<$event_type>)
                        .add_system(write_events::<$event_type>)
                    )*
                ;
            }
        }
    }
}

network_plugin!(
    NodeMetricEvent, "node_metric",
    LayerMetricEvent, "layer_metric",
    NetworkMetricEvent, "network_metric",
    NodeChildrenMetricEvent, "node_as_children_metric"
);

