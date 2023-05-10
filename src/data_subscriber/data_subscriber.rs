use bevy::prelude::{ResMut, Resource};
use crate::data_subscriber::kafka_data_subscriber::{EventReceiver, KafkaClientProvider};
use crate::data_subscriber::metric_event::NetworkEvent;

pub trait MessageClientProvider: Resource {}

pub trait DataSubscriber<E,P>
where
    E: NetworkEvent + 'static,
    P: MessageClientProvider
{
   fn subscribe(
       consumer: ResMut<P>,
       receiver_handler: ResMut<EventReceiver<E>>
   );
}