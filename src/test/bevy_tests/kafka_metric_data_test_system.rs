use std::cell::{Cell, RefCell};
use std::env;
use std::env::VarError;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::hash::Hash;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::{Arc, mpsc, Mutex};
use std::time::Duration;
use bevy::prelude::{Commands, Component, Condition, error, Events, EventWriter, info, Res, ResMut, Resource, World};
use bevy::tasks::AsyncComputeTaskPool;
use std::collections::HashMap;
use bevy::utils::petgraph::visit::Walker;
use rdkafka::{ClientConfig, Message, Offset, Timestamp, TopicPartitionList};
use rdkafka::admin::AdminClient;
use rdkafka::client::DefaultClientContext;
use rdkafka::config::{FromClientConfig, FromClientConfigAndContext};
use rdkafka::consumer::{Consumer, DefaultConsumerContext, StreamConsumer};
use rdkafka::error::{KafkaError, KafkaResult};
use rdkafka::message::{BorrowedMessage, OwnedHeaders};
use rdkafka::producer::{DefaultProducerContext, FutureProducer, FutureRecord};
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use rdkafka::util::Timeout;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::mpsc::Receiver;
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use tokio::time::timeout;
use crate::config::ConfigurationProperties;
use crate::data_subscriber::data_subscriber::{DataSubscriber, MessageClientProvider};
use crate::data_subscriber::kafka_data_subscriber::KafkaClientProvider;
use crate::data_subscriber::metric_event::{NetworkEvent, NetworkMetricEvent, NetworkMetricsServiceEvent, NodeMetricEvent};
use crate::metrics::network_metrics::{Metric, MetricChildNodes};
use crate::network::{Layer, Network, Node};
use crate::util::{get_create_runtime, run_blocking};


pub(crate) fn test_kafka_data<E: NetworkEvent>(
    mut consumer: ResMut<KafkaClientProvider>,
)
{
    info!("Creating test kafka data");
    let producer = run_blocking(consumer.get_producer());
    producer
        .map(|mut producer| {
            info!("Found producer.");

            let sent = send_message(&mut producer, vec![0.0, 1.0, 2.0, 3.0]);
            let sent_two = send_message(&mut producer, vec![0.0, 1.0, 2.0, 3.0]);

            info!("Sent kafka message: {:?}", sent);
        }).unwrap();
}

fn send_message(producer: &mut FutureProducer, vec1: Vec<f32>) -> OwnedDeliveryResult {
    let network_metric_event = NodeMetricEvent {
        shape: vec![3],
        data: Mutex::new(Some(vec1)),
        metric_name: "metric".to_string(),
        included: vec![],
        columns: Some(HashMap::from([
            ("first".to_string(), 0),
            ("second".to_string(), 1),
            ("third".to_string(), 2),
            ("fourth".to_string(), 2)
        ])),
    };

    let json_str_result = serde_json::to_string(&network_metric_event);
    let result = json_str_result.unwrap();
    info!("Sending kafka message");

    let record = FutureRecord::to("node_metric_one")
        .key("metric")
        .payload(result.as_bytes())
        .partition(0)
        .timestamp(Timestamp::now().to_millis().unwrap())
        .headers(OwnedHeaders::new());

    let sent = run_blocking(producer.send(record, Timeout::After(Duration::new(3, 0))));
    sent
}
