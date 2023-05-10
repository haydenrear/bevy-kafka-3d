use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use kafka::client::fetch::Data;

use crate::data_subscriber::kafka_data_subscriber::{KafkaClientProvider, write_events, EventReceiver, KafkaMessageSubscriber};
use crate::data_subscriber::metric_event::{LayerMetricEvent, NodeChildrenMetricEvent, NetworkMetricEvent, NodeMetricEvent, NetworkEvent};
use crate::metrics::network_metrics::{Metric, MetricChildNodes};
use crate::network::{Layer, Network, Node};
use crate::data_subscriber::data_event_reader::MetricsState;
use crate::data_subscriber::data_subscriber::DataSubscriber;
use crate::data_subscriber::network_metadata_event::NetworkMetadataEvent;


pub struct DataSubscriberPlugin;

// impl Plugin for DataSubscriberPlugin {
//     fn build(&self, app: &mut App) {
//         app.insert_resource(KafkaClientProvider::default())
//             .insert_resource::<EventReceiver<NodeMetricEvent, Node>>(EventReceiver::default())
//             .add_event::<NodeMetricEvent>()
            // .add_event::<LayerMetricEvent>()
            // .add_event::<NetworkMetricEvent>()
            // .add_event::<NodeChildrenMetricEvent>()
            // .add_startup_system(consume_kafka_messages::<NodeMetricEvent>)
            // .add_startup_system(consume_kafka_messages::<LayerMetricEvent>)
            // .add_startup_system(consume_kafka_messages::<NetworkMetricEvent>)
            // .add_startup_system(consume_kafka_messages::<NodeChildrenMetricEvent>)
            // .add_system(write_events::<NodeMetricEvent, Node>)
//         ;
//     }
// }


macro_rules! network_plugin {
    ($($event_type:ident, $component_ty:ty),*) => {
        impl Plugin for DataSubscriberPlugin {
            fn build(&self, app: &mut App) {
                app.insert_resource(KafkaClientProvider::default())
                    $(
                        .add_event::<$event_type>()
                        .insert_resource::<EventReceiver<$event_type>>(EventReceiver::default())
                        .add_startup_system(KafkaMessageSubscriber::<$event_type>::subscribe)
                        .add_system(write_events::<$event_type>)
                    )*
                    .insert_resource(MetricsState::default())
                    .add_event::<NetworkMetadataEvent>()
                ;
            }
        }
    }
}


network_plugin!(
    NodeMetricEvent, Node,
    LayerMetricEvent, Layer,
    NetworkMetricEvent, Network,
    NodeChildrenMetricEvent, MetricChildNodes
);