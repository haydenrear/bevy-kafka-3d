use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::time::Duration;
use bevy::asset::FileAssetIo;
use bevy::core_pipeline::core_3d::{Core3dPlugin, Transparent3d};
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::gltf::GltfPlugin;
use bevy::log::LogPlugin;
use bevy::pbr::{MeshPipeline, MeshRenderPlugin, PbrPlugin};
use bevy::prelude::*;
use bevy::render::mesh::MeshPlugin;
use bevy::render::pipelined_rendering::{PipelinedRenderingPlugin, RenderToMainAppReceiver};
use bevy::render::render_phase::DrawFunctions;
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::sprite::SpritePlugin;
use bevy::tasks::{AsyncComputeTaskPool, TaskPool, TaskPoolBuilder};
use bevy::utils::HashMap;
use bevy::winit::WinitPlugin;
use ndarray::{ArrayBase, Ix1, OwnedRepr};
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::{Message, Timestamp};
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::FutureRecord;
use rdkafka::util::Timeout;
use tokio::runtime::{Handle, Runtime};
use tokio::spawn;
use tokio::task::{LocalSet, spawn_blocking, spawn_local};
use crate::config::ConfigurationProperties;
use crate::config::kafka::KafkaConfiguration;
use crate::data_subscriber::data_event_reader::{MetricsState, read_metric_events};
use crate::data_subscriber::data_subscriber::DataSubscriber;
use crate::data_subscriber::kafka_data_subscriber::{EventReceiver, KafkaClientProvider, KafkaMessageSubscriber, write_events};
use crate::data_subscriber::metric_event::{NetworkMetricEvent, NodeMetricEvent};
use crate::data_subscriber::network_metadata_event::NetworkMetadataEvent;
use crate::graph::draw_graph_points::draw_graph_points;
use crate::graph::{GraphConfigurationResource, GraphData, SeriesStep};
use crate::graph::radial::RadialGraphPoints;
use crate::graph::setup_graph::{graph_points_generator, setup_graph};
use crate::lines::line_list::LineMaterial;
use crate::menu::config_menu_event::interaction_config_event_writer::ConfigOptionContext;
use crate::metrics::network_metrics::Metric;
use crate::network::Node;
use crate::test::bevy_tests::default_plugins::NoRenderBevyIntegrationTestPlugin;
use crate::test::bevy_tests::graphing_tests::mock_metric_data_test_system::{TestEventGeneratingResource, write_fake_metric_network_events};
use crate::util::run_blocking;


#[tokio::test]
async fn test_draw_graph_points_1_x_1() {
    let mut app = App::new();
    let app = create_app(1, &mut app);

    let metric = app.world.resource::<MetricsState>()
        .entities.get("metric").unwrap();

    let node_metric = app.world.get::<Metric<Node>>(metric.0).unwrap();
    let first_column = node_metric.historical.retrieve_historical_1d("0");

    println!("{:?} is first colum.", &first_column);
    println!("{:?} is first asserted first column.", &first_column[0]);
    println!("{:?} is metric entity.", metric.0);

    let metric_children = app.world.get::<Children>(metric.0);

    assert!(metric_children.is_some());
    assert!(metric_children.unwrap().iter().any(|c| app.world.get::<SeriesStep>(*c).is_some()));

}

#[tokio::test]
async fn test_draw_graph_points_1_x_20() {
    let mut app = App::new();

    let app = create_app(20, &mut app);

    let metric = app.world.resource::<MetricsState>()
        .entities.get("metric").unwrap();

    let node_metric = app.world.get::<Metric<Node>>(metric.0).unwrap();
    let first_column = node_metric.historical.retrieve_historical_1d("0");

    println!("{:?} is first colum.", &first_column);
    println!("{:?} is first asserted first column.", &first_column[0]);
    println!("{:?} is metric entity.", metric.0);

    let metric_children = app.world.get::<Children>(metric.0);

    assert!(metric_children.is_some());
    assert!(metric_children.unwrap().iter().any(|c| app.world.get::<SeriesStep>(*c).is_some()));

}


fn create_app<'a>(dim: usize, app: &'a mut App) -> &'a mut App {

    let mut config_properties = ConfigurationProperties::default();

    AsyncComputeTaskPool::init(|| { TaskPool::default() });

    let mut app = app
        .insert_resource(GraphData::default())
        .insert_resource(MetricsState::default())
        .insert_resource(config_properties)
        .insert_resource(ConfigOptionContext::default())
        .insert_resource(TestEventGeneratingResource::new(dim))
        .insert_resource(GraphConfigurationResource::<Node>::default())
        .add_plugins(NoRenderBevyIntegrationTestPlugin)
        .add_plugin(MaterialPlugin::<LineMaterial>::default())
        .add_startup_system(setup_graph)
        .add_system(write_fake_metric_network_events)
        .add_system(read_metric_events::<NodeMetricEvent, Node>)
        .add_system(draw_graph_points::<Node, RadialGraphPoints, LineMaterial>)
        .add_system(graph_points_generator::<Node>)
        .add_event::<NodeMetricEvent>()
        .add_event::<NetworkMetadataEvent>();

    app.update();
    app.update();
    app.update();
    app.update();
    app
}

pub(crate) fn to_assert(metric: &(Entity, u64), first_column: &Vec<ArrayBase<OwnedRepr<f32>, Ix1>>, x: f32) -> Vec<f32> {
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