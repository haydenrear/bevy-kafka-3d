use std::env;
use std::env::VarError;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::{Arc, mpsc, Mutex};
use std::time::Duration;
use bevy::prelude::{Commands, Condition, error, Events, EventWriter, Res, ResMut, Resource, World};
use bevy::tasks::AsyncComputeTaskPool;
use bevy::utils::petgraph::visit::Walker;
use kafka::client::{FetchOffset, KafkaClient};
use kafka::consumer::Consumer;
use kafka::producer::Producer;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Receiver;
use tokio::sync::Semaphore;
use tokio::time::timeout;

#[derive(Resource, Default)]
pub(crate) struct KafkaReceiver<T: NetworkMetricsServiceEvent + Send + Sync + 'static> {
    receiver: Option<Receiver<T>>
}

pub trait NetworkMetricsServiceEvent: for<'a> Deserialize<'a> + Send + Sync {
    fn topic_matcher() -> &'static str;
}

macro_rules! network_events {
    ($($event_type:ident, $event_lit:literal),*) => {
        $(

            #[derive(Serialize, Deserialize, Default, Debug, Clone)]
            pub struct $event_type {
            }

            impl NetworkMetricsServiceEvent for $event_type {
                fn topic_matcher() -> &'static str {
                    return $event_lit;
                }
            }
        )*
    }
}

network_events!(
    NodeMetricEvent, "node_metric",
    LayerMetricEvent, "layer_metric",
    NetworkMetricEvent, "network_metric",
    NodeChildrenMetricEvent, "node_as_children_metric"
);

#[derive(Resource, Debug)]
pub struct KafkaClientProvider {
    kafka_client: KafkaClient,
    hosts: Vec<String>,
    client_id: String,
    group_id: String,
    num_consumers_per_event: u8,
    consumers: Vec<Consumer>
}

#[derive(Deserialize, Default)]
pub struct ConfigurationProperties {
    kafka: KafkaConfiguration
}

#[derive(Deserialize)]
pub struct KafkaConfiguration {
    hosts: Vec<String>,
    consumer_group_id: String,
    client_id: String
}

impl Default for KafkaConfiguration {
    fn default() -> Self {
        Self {
            hosts: vec!["localhost:9092".to_string()],
            consumer_group_id: "consumer".to_string(),
            client_id: "nn-fe".to_string()
        }
    }
}

impl Default for KafkaClientProvider {
    fn default() -> Self {
        let config = Self::read_config();
        let hosts = config.kafka.hosts;
        let group_id = config.kafka.consumer_group_id;
        let client_id = config.kafka.client_id;
        let mut kc = KafkaClient::new(hosts.clone());
        Self {
            kafka_client: kc,
            group_id,
            client_id,
            hosts,
            num_consumers_per_event: 1,
            consumers: vec![],
        }
    }
}

impl KafkaClientProvider {
    fn get_consumer(&self, topics: Vec<String>) -> kafka::Result<Consumer> {
        let mut consumer_builder = kafka::consumer::Consumer::from_hosts(self.hosts.clone())
            .with_group(self.group_id.to_string())
            .with_client_id(self.client_id.clone())
            .with_fallback_offset(FetchOffset::Earliest);

        for topic in topics.into_iter() {
            consumer_builder = consumer_builder.with_topic(topic);
        }

        consumer_builder
            .create()
    }

    fn get_producer(&self) -> kafka::Result<Producer> {
        Producer::from_hosts(self.hosts.clone())
            .with_client_id(self.client_id.clone())
            .create()
    }

    fn read_config() -> ConfigurationProperties {
        let config_file = env::var("CONFIG_PROPS")
            .or(Ok::<String, VarError>("resources/config.toml".to_string()))
            .unwrap();
        let config = read_to_string(Path::new(&config_file))
            .map(|toml| toml::from_str::<ConfigurationProperties>(toml.as_str()).ok())
            .or_else(|e| {
                error!("Error reading configuration properties: {:?}.", e);
                Ok::<Option<ConfigurationProperties>, toml::de::Error>(Some(ConfigurationProperties::default()))
            })
            .ok()
            .flatten()
            .unwrap();
        config
    }
}

pub(crate) fn write_events<E: NetworkMetricsServiceEvent + Send + Sync + Debug + 'static>(
    mut event_writer: EventWriter<E>,
    mut receiver_handler: ResMut<KafkaReceiver<E>>
) {
    if receiver_handler.receiver.is_none() {
        return;
    }
    if let Ok(event) = receiver_handler.receiver.as_mut().unwrap().try_recv() {
        event_writer.send(event);
    }
}

pub(crate) fn consume_kafka_messages<E: NetworkMetricsServiceEvent + Send + Sync + Debug + 'static>(
    mut consumer: ResMut<KafkaClientProvider>,
    mut receiver_handler: ResMut<KafkaReceiver<E>>
) {
    let topics = vec![E::topic_matcher().to_string()];
    let mut consumers = vec![];
    let mut task_pool = AsyncComputeTaskPool::get();


    for _ in 0..consumer.num_consumers_per_event {
        let _ = consumer.get_consumer(topics.clone())
            .map(|consumer| {
                consumers.push(consumer);
            })
            .or_else(|e| {
                error!("Failed to create Kafka consumer for topic {:?}: {:?}", &topics, e);
                Ok::<(), kafka::Error>(())
            })
            .ok();
    }

    let (mut tx, mut rx) = tokio::sync::mpsc::channel::<E>(16);

    let mut rx: Receiver<E> = rx;

    let _ = std::mem::replace(&mut receiver_handler.receiver, Some(rx));

    let tx = Arc::new(tx);

    consumers.into_iter().for_each(|mut consumer| {
        let tx = tx.clone();
        let _ = task_pool.spawn(async move {
            let tx = tx.clone();
            loop {
                if let Ok(message_set) = consumer.poll() {
                    for message_set in message_set.iter() {
                        for message in message_set.messages().iter() {
                            let event = match serde_json::from_slice::<E>(message.value) {
                                Ok(event) => event,
                                Err(e) => {
                                    error!("Error deserializing event: {:?}.", e);
                                    continue;
                                }
                            };
                            let _ = tx.send(event)
                                .await
                                .or_else(|e| {
                                    error!("Error sending event: {}.", e.to_string());
                                    Err(e)
                                });
                        }
                    }
                }
            }
        });
    });

}