use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;

use crate::data_subscriber::kafka_data_subscriber::{EventReceiver, KafkaClientProvider, KafkaMessageSubscriber, write_events};
use crate::data_subscriber::metric_event::{LayerMetricEvent, NetworkEvent, NetworkMetricEvent, NodeChildrenMetricEvent, NodeMetricEvent};
use crate::metrics::network_metrics::Metric;
use crate::network::{Layer, MetricChildNodes, Network, Node};
use crate::data_subscriber::network_metadata_event::NetworkMetadataEvent;
use crate::data_subscriber::metric_event::MetricsState;
use crate::data_subscriber::data_subscriber::DataSubscriber;

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