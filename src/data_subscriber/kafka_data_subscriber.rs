use std::cell::{Cell, RefCell};
use std::env;
use std::env::VarError;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::future::Future;
use std::hash::Hash;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::{Arc, mpsc, Mutex};
use std::time::Duration;
use bevy::log::debug;
use bevy::prelude::{Commands, Component, Condition, error, Events, EventWriter, info, Res, ResMut, Resource, World};
use bevy::tasks::AsyncComputeTaskPool;
use bevy::utils::HashMap;
use bevy::utils::petgraph::visit::Walker;
use rdkafka::{ClientConfig, Message, Offset, TopicPartitionList};
use rdkafka::admin::AdminClient;
use rdkafka::client::DefaultClientContext;
use rdkafka::config::{FromClientConfig, FromClientConfigAndContext};
use rdkafka::consumer::{Consumer, DefaultConsumerContext, StreamConsumer};
use rdkafka::error::{KafkaError, KafkaResult};
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::{DefaultProducerContext, FutureProducer};
use tokio::runtime::{Handle, Runtime};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use tokio::time;
use tokio::time::{timeout, Timeout};
use tokio::time::error::Elapsed;
use crate::config::ConfigurationProperties;
use crate::data_subscriber::data_subscriber::{DataSubscriber, MessageClientProvider};
use crate::data_subscriber::metric_event::{NetworkEvent, NetworkMetricsServiceEvent};
use crate::metrics::network_metrics::Metric;
use crate::network::{Layer, MetricChildNodes, Network, Node};
use crate::util::{get_create_runtime, run_blocking};

#[derive(Resource, Default)]
pub struct EventReceiver<T>
where
    T: NetworkEvent {
    receiver: Option<Receiver<T>>
}

#[derive(Resource)]
pub struct KafkaClientProvider {
    kafka_client: Option<AdminClient<DefaultClientContext>>,
    hosts: Vec<String>,
    client_id: String,
    group_id: String,
    num_consumers_per_event: u8,
    consumers: KafkaConsumerContainer
}

#[derive(Default, Clone)]
pub struct KafkaConsumerContainer {
    consumers: Arc<Mutex<Vec<(Arc<StreamConsumer>, Vec<String>)>>>
}

impl MessageClientProvider for KafkaClientProvider {}

impl Default for KafkaClientProvider {
    fn default() -> Self {
        let properties = ConfigurationProperties::read_config();
        let client_config = Self::admin_client_config_properties_set(&properties, properties.kafka.hosts.join(","));
        let mut kc = AdminClient::from_config(&client_config)
            .or_else(|e| {
                error!("Error connecting to kafka with AdminClient: {:?}", e);
                Err(e)
            })
            .ok();
        Self {
            kafka_client: kc,
            group_id: properties.kafka.consumer_group_id,
            client_id: properties.kafka.client_id,
            hosts: properties.kafka.hosts,
            num_consumers_per_event: 1,
            consumers: Default::default(),
        }
    }
}

impl KafkaClientProvider {
    pub(crate) async fn get_consumer(&mut self, topics: Vec<&str>) -> Result<StreamConsumer, KafkaError> {
        let client_config = self.admin_client_config_properties();

        let consumer: Result<StreamConsumer, KafkaError> =
            client_config.create_with_context(DefaultConsumerContext);


        let consumer = consumer.map(|consumer| {
            let topic_to_subcribe = Self::fetch_topic_patterns(&topics, &consumer);
            info!("Subscribing to topics: {:?}.", &topics);
            let subscribe_topics_slice = topic_to_subcribe.iter()
                .map(|t| t.as_str())
                .collect::<Vec<&str>>();

            Self::subscribe_to_topics(&consumer, subscribe_topics_slice.as_slice());

            consumer

        });
        consumer
    }

    fn subscribe_to_topics(consumer: &StreamConsumer, subscribe_topics_slice: &[&str]) {
        let _ = consumer.subscribe(subscribe_topics_slice)
            .or_else(|e| {
                error!("Error subscribing to topics: {:?}.", e);
                Err(e)
            });

        subscribe_topics_slice.iter().for_each(|topic| {
            let mut partitions = TopicPartitionList::new();
            partitions.add_partition(topic, 0);
            let _ = partitions.set_all_offsets(Offset::Beginning)
                .or_else(|e| {
                    error!("Error assigning partitions offset {}.", topic);
                    Err(e)
                });
            let _ = consumer.assign(&partitions)
                .or_else(|e| {
                    error!("Error assigning partitions for {}.", topic);
                    Err(e)
                });
        });
    }

    fn fetch_topic_patterns(topics: &Vec<&str>, consumer: &StreamConsumer) -> Vec<String> {
        let mut topic_to_subcribe = vec![];
        let _ = consumer.client()
            .fetch_metadata(None, rdkafka::util::Timeout::After(Duration::from_secs(3)))
            .map(|all_topics_metadata| {
                all_topics_metadata.topics().iter()
                    .filter(|topic| topics.iter()
                        .any(|topic_match| matches!(topic.name(), topic_match))
                    )
                    .map(|topic| topic.name().to_string())
                    .for_each(|topic| topic_to_subcribe.push(topic));
            })
            .or_else(|e| {
                error!("Could not fetch topics to determine which topics to subscribe: {:?}.", e);
                Err(e)
            });
        topic_to_subcribe
    }

    pub(crate) async fn get_producer(&self) -> Result<FutureProducer, KafkaError> {
        let mut client_config = self.admin_client_config_properties();
        FutureProducer::from_config(&client_config)
    }

    pub(crate) fn new(port: u16) -> Self {
        let properties = ConfigurationProperties::read_config();
        let mut client_config = Self::admin_client_config_properties_set(&properties, format!("localhost:{}", port));
        let mut kc = AdminClient::from_config(&client_config)
            .or_else(|e| {
                error!("Error connecting to kafka with AdminClient: {:?}", e);
                Err(e)
            })
            .ok();
        Self {
            kafka_client: kc,
            group_id: properties.kafka.consumer_group_id,
            client_id: properties.kafka.client_id,
            hosts: vec![format!("localhost:{}", port)],
            num_consumers_per_event: 1,
            consumers: Default::default(),
        }
    }

    pub(crate) fn get_admin_client(&self) -> &Option<AdminClient<DefaultClientContext>> {
        &self.kafka_client
    }

    fn admin_client_config_properties(&self) -> ClientConfig {
        let mut client_config = ClientConfig::new();
        client_config.set("bootstrap.servers", self.hosts.join(","));
        client_config.set("group.id", self.group_id.clone());
        client_config.set("client.id", self.client_id.clone());
        client_config
    }

    fn admin_client_config_properties_set(config: &ConfigurationProperties, hosts: String) -> ClientConfig {
        let mut client_config = ClientConfig::new();
        client_config.set("bootstrap.servers", hosts);
        client_config.set("group.id", config.kafka.consumer_group_id.clone());
        client_config.set("client.id", config.kafka.client_id.clone());
        client_config
    }
}

pub(crate) fn write_events<E>
(
    mut event_writer: EventWriter<E>,
    mut receiver_handler: ResMut<EventReceiver<E>>
)
where E: NetworkEvent + 'static + Debug
{
    info!("Checking events.");
    if receiver_handler.receiver.is_none() {
        error!("Received event but there was no receiver handler set.");
        return;
    }
    run_blocking(async {
        match time::timeout(Duration::from_secs(3), async {
            if let Some(event) = receiver_handler.receiver.as_mut().unwrap().recv().await {
                info!("{:?} is event.", &event);
                event_writer.send(event);
            }
        }).await {
            Ok(_) => {}
            Err(_) => {}
        }
    });
}

pub struct KafkaMessageSubscriber<E>
    where E: NetworkEvent + 'static
{
    phantom: PhantomData<E>,
}

impl <E> DataSubscriber<E, KafkaClientProvider> for KafkaMessageSubscriber<E>
    where E: NetworkEvent + 'static + Debug
{

    fn subscribe(
        mut consumer_config: ResMut<KafkaClientProvider>,
        mut receiver_handler: ResMut<EventReceiver<E>>,
    )
    {

        let topics = vec![E::topic_matcher()];
        let mut consumers = vec![];
        let mut task_pool = AsyncComputeTaskPool::get();

        for _ in 0..consumer_config.num_consumers_per_event {
            run_blocking(async {
                let _ = consumer_config.get_consumer(topics.clone())
                    .await
                    .map(|consumer| {
                        info!("Created consumer for {:?}.", &topics);
                        consumers.push(Arc::new(consumer));
                    })
                    .or_else(|e| {
                        error!("Failed to create Kafka consumer for topic {:?}: {:?}", &topics, e);
                        Ok::<(), KafkaError>(())
                    })
                    .ok();
            })
        }

        let _ = consumer_config.consumers.consumers
            .lock()
            .map(|mut c| {
                let consumer = consumers.iter()
                    .map(|c| (c.clone(), vec![E::topic_matcher().to_string()]))
                    .collect::<Vec<(Arc<StreamConsumer>, Vec<String>)>>();
                c.extend(consumer);
            })
            .or_else(|e| {
                error!("Error saving kafka consumers: {:?}", e);
                Err(e)
            });

        let (mut tx, mut rx) = tokio::sync::mpsc::channel::<E>(16);

        let mut rx: Receiver<E> = rx;

        let _ = std::mem::replace(&mut receiver_handler.receiver, Some(rx));

        let tx = Arc::new(tx);

        info!("Initializing kafka subscriber for topics: {:?}.", topics);

        consumers.into_iter().for_each(|mut consumer| {
            let tx = tx.clone();
            task_pool.spawn(async move {
                info!("Created task to subscribe to messages.");
                let tx = tx.clone();
                loop {
                    match consumer.recv().await {
                        Ok(message_set) => {
                            if let Some(payload) = message_set.payload() {
                                let event = match serde_json::from_slice::<E>(payload) {
                                    Ok(event) => event,
                                    Err(e) => {
                                        error!("Error deserializing event: {:?}.", e);
                                        continue;
                                    }
                                };
                                info!("Sending message");
                                let _ = tx.send(event)
                                    .await
                                    .or_else(|e| {
                                        error!("Error sending event: {}.", e.to_string());
                                        Err(e)
                                    });
                            }
                        },
                        Err(kafka_error) => {
                            error!("Error receiving consumer message: {:?}.", kafka_error);
                        }
                    }
                }
            }).detach();
        });

    }
}
