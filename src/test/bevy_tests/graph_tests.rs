use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use std::time::Duration;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, TaskPool, TaskPoolBuilder};
use bevy::utils::HashMap;
use ndarray::{ArrayBase, Ix1, OwnedRepr};
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::{Message, Timestamp};
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::FutureRecord;
use rdkafka::util::Timeout;
use testcontainers::clients::Cli;
use testcontainers::core::WaitFor;
use testcontainers::Image;
use testcontainers::images::kafka::Kafka;
use tokio::runtime::{Runtime, Handle};
use tokio::spawn;
use tokio::task::{LocalSet, spawn_blocking, spawn_local};
use crate::config::{ConfigurationProperties, KafkaConfiguration};
use crate::data_subscriber::data_event_reader::{MetricsState, read_metric_events};
use crate::data_subscriber::data_subscriber::DataSubscriber;
use crate::data_subscriber::kafka_data_subscriber::{EventReceiver, KafkaClientProvider, KafkaMessageSubscriber, write_events};
use crate::data_subscriber::metric_event::{NetworkMetricEvent, NodeMetricEvent};
use crate::data_subscriber::network_metadata_event::NetworkMetadataEvent;
use crate::metrics::network_metrics::Metric;
use crate::network::Node;
use crate::test::bevy_tests::kafka_testcontainers;
use crate::test::bevy_tests::kafka_testcontainers::{create_topics, KafkaContainer};
use crate::test::bevy_tests::kafka_metric_data_test_system::test_kafka_data;
use crate::test::bevy_tests::mock_metric_data_test_system::write_fake_metric_network_events;
use crate::util::run_blocking;

#[tokio::test]
async fn test_load_metric_from_kafka() {
    let cli = Cli::default();
    let kafka_container = kafka_testcontainers::start_kafka_container(vec!["node_metric_one"], &cli)
        .await;

    let mut app = App::new();

    AsyncComputeTaskPool::init(|| { TaskPool::default() });

    let mut config_properties = ConfigurationProperties::default();
    let client_provider = KafkaClientProvider::new(kafka_container.port);

    let mut app = app
        .add_startup_system(KafkaMessageSubscriber::<NodeMetricEvent>::subscribe)
        .add_startup_system(test_kafka_data::<NodeMetricEvent>)
        .add_system(write_events::<NodeMetricEvent>)
        .add_system(read_metric_events::<NodeMetricEvent, Node>)
        .add_plugin(LogPlugin::default())
        .insert_resource::<EventReceiver<NodeMetricEvent>>(EventReceiver::default())
        .insert_resource(MetricsState::default())
        .insert_resource(client_provider)
        .insert_resource(config_properties)
        .add_event::<NodeMetricEvent>()
        .add_event::<NetworkMetadataEvent>();

    app.update();
    app.update();
    app.update();
    app.update();

    assert!(wait_for::wait_for::wait_async::WaitFor::wait_for(Duration::new(4, 0), &|| {
        let metrics = &app.world.resource::<MetricsState>()
            .entities;
        metrics.len()
    }, &|len| {
        println!("{}", len);
        len != 0
    }));

    assert!(wait_for::wait_for::wait_async::WaitFor::wait_for(Duration::new(3, 0), &|| {
        let metrics = &app.world.resource::<MetricsState>()
            .entities
            .get("metric")
            .unwrap()
            .1;
        *metrics
    }, &|len| {
        println!("{}", len);
        len != 0
    }));

    let metric = app.world.resource::<MetricsState>()
        .entities.get("metric").unwrap();

    let node_metric = app.world.get::<Metric<Node>>(metric.0).unwrap();
    let first_column = node_metric.historical.retrieve_historical_1d("first");
    let to_assert = to_assert(metric, &first_column, 0 as f32);

    println!("{:?} is first colum.", &first_column);
    println!("{:?} is first asserted first column.", &first_column[0]);

    assert_ne!(metric.1, 0);
    assert_eq!(to_assert.as_slice(), vec![0 as f32; (metric.1 as usize)  + 1].as_slice());

}


#[tokio::test]
async fn test_draw_graph_points() {

    let mut app = App::new();

    AsyncComputeTaskPool::init(|| { TaskPool::default() });

    let mut config_properties = ConfigurationProperties::default();

    let mut app = app
        .add_system(write_fake_metric_network_events)
        .add_system(read_metric_events::<NodeMetricEvent, Node>)
        .add_plugin(LogPlugin::default())
        .insert_resource(MetricsState::default())
        .insert_resource(config_properties)
        .add_event::<NodeMetricEvent>()
        .add_event::<NetworkMetadataEvent>();

    app.update();
    app.update();
    app.update();
    app.update();

    assert!(wait_for::wait_for::wait_async::WaitFor::wait_for(Duration::new(4, 0), &|| {
        let metrics = &app.world.resource::<MetricsState>()
            .entities;
        metrics.len()
    }, &|len| {
        println!("{}", len);
        len != 0
    }));

    assert!(wait_for::wait_for::wait_async::WaitFor::wait_for(Duration::new(3, 0), &|| {
        let metrics = &app.world.resource::<MetricsState>()
            .entities
            .get("metric")
            .unwrap()
            .1;
        *metrics
    }, &|len| {
        println!("{}", len);
        len != 0
    }));

    let metric = app.world.resource::<MetricsState>()
        .entities.get("metric").unwrap();

    let node_metric = app.world.get::<Metric<Node>>(metric.0).unwrap();
    let first_column = node_metric.historical.retrieve_historical_1d("first");
    let to_assert = to_assert(metric, &first_column, 0 as f32);

    println!("{:?} is first colum.", &first_column);
    println!("{:?} is first asserted first column.", &first_column[0]);

    assert_ne!(metric.1, 0);
    assert_eq!(to_assert.as_slice(), vec![0 as f32; (metric.1 as usize)  + 1].as_slice());

}

fn to_assert(metric: &(Entity, u64), first_column: &Vec<ArrayBase<OwnedRepr<f32>, Ix1>>, x: f32) -> Vec<f32> {
    let mut to_assert: Vec<f32> = vec![x; (metric.1 as usize) + 1];
    first_column.iter().enumerate()
        .map(|col| {
            let slice = col.1.as_slice().unwrap();
            (col.0, slice)
        })
        .for_each(|(index, arr)| {
            to_assert[index] = arr[0];
        });
    to_assert
}