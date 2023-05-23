use std::collections::{BTreeSet, HashMap};
use std::fmt::Debug;
use bevy::log::{error, info};
use bevy::pbr::PbrBundle;
use bevy::prelude::{BuildChildren, Color, Commands, Component, Entity, EventReader, Mut, Query, Res, ResMut, Resource};
use bevy_mod_picking::PickableBundle;
use crate::config::ConfigurationProperties;
use crate::cursor_adapter::PickableComponent;
use crate::data_subscriber::metric_event::{MetricsState, NetworkMetricsServiceEvent};
use crate::graph::{DataSeries, GraphConfigurationResource, GraphDim, GraphDimComponent, GraphingMetricsResource, GridAxis};
use crate::menu::graphing_menu::graph_menu::GraphingPotential;
use crate::metrics::network_metrics::{Metric, MetricType, MetricTypeMatcher};
use crate::ndarray::get_arr_from_vec;
use crate::util::gen_color_from_list;


#[derive(Component, Default, Debug)]
pub struct HistoricalUpdated;

/// When messages are received from Kafka, the are sent as NetworkMetricsServiceEvents. They are
/// retrieved here and the data is added to the historical data. The data is then available in memory
/// to be displayed. It will therefore show up in the menu, and the user sets how to display the data.
/// There will be a default for the various metrics.
pub fn read_metric_events<T, U>(
    mut commands: Commands,
    mut event_read: EventReader<T>,
    mut metrics_lookup: ResMut<MetricsState>,
    config_properties: Res<ConfigurationProperties>,
    mut graph_dim_config: ResMut<GraphConfigurationResource<U>>,
    mut graph_config: ResMut<GraphingMetricsResource>,
    mut component_query: Query<(Entity, &mut Metric<U>)>,
)
    where
        T: NetworkMetricsServiceEvent<U> + 'static + Debug,
        U: Component + 'static
{
    for mut event in event_read.iter() {
        let metric_name = event.metric_name();
        info!("Receiving network event: {}.", metric_name);
        if metrics_lookup.entities.contains_key(metric_name) {
            add_data_to_current_metric(&mut commands, &mut metrics_lookup, &mut component_query, event, metric_name);
        } else {
            create_new_metric(&mut commands, &mut metrics_lookup, &config_properties, &mut graph_dim_config, &mut graph_config, event, metric_name);
        }
        metrics_lookup.increment_entity(metric_name);
    }
}

fn create_new_metric<U, T>(
    mut commands: &mut Commands,
    mut metrics_lookup: &mut ResMut<MetricsState>,
    config_properties: &Res<ConfigurationProperties>,
    mut graph_dim_config: &mut ResMut<GraphConfigurationResource<U>>,
    mut graph_config: &mut ResMut<GraphingMetricsResource>,
    mut event: &T,
    metric_name: &str,
)
    where
        U: Component + 'static,
        T: NetworkMetricsServiceEvent<U> + 'static + Debug,
{
    info!("Adding metric to resource.");
    for (matcher, to_match) in config_properties.metrics.metric_type.iter() {
        let to_match = to_match.as_str();
        if matches!(metric_name, to_match) {
            create_add_metric(&mut commands, &mut metrics_lookup, &config_properties, &mut graph_dim_config, &mut graph_config,  event, metric_name, matcher);
            break;
        }
    }
}

fn add_data_to_current_metric<T, U>(
    mut commands: &mut Commands, metrics_lookup:
    &mut ResMut<MetricsState>,
    component_query: &mut Query<(Entity, &mut Metric<U>)>,
    mut event: &T,
    metric_name: &str,
)
    where
        T: NetworkMetricsServiceEvent<U> + 'static + Debug,
        U: Component
{
    let (entity, timestep) = metrics_lookup.entities
        .get(metric_name)
        .unwrap();
    let _ = component_query.get_mut(*entity)
        .as_mut()
        .map(|(entity, metric)| extend_historical(&mut commands, event, timestep, entity, metric))
        .or_else(|e| {
            error!("Could not extend metric: {:?}.", e);
            Err(e)
        });
}

fn create_add_metric<U, T>(
    mut commands: &mut Commands,
    mut metrics_lookup: &mut ResMut<MetricsState>,
    config_properties: &Res<ConfigurationProperties>,
    mut graph_dim_config: &mut ResMut<GraphConfigurationResource<U>>,
    mut graph_config: &mut ResMut<GraphingMetricsResource>,
    mut event: &T,
    metric_name: &str,
    matcher: &MetricTypeMatcher,
)
    where
        U: Component + 'static,
        T: NetworkMetricsServiceEvent<U> + Debug +  'static
{
    let columns = event.get_columns()
        .or(Some(HashMap::new()))
        .unwrap();

    let mut metric = create_metric_struct(event, matcher, columns);

    add_historical(event, &mut metric);

    let columns = get_graph_dims(&config_properties, &mut metric);

    add_metric_to_world(&mut commands, &mut metrics_lookup, &mut graph_dim_config, &mut graph_config, metric_name, metric, columns);
}

fn create_metric_struct<U, T>(
    mut event: &T,
    matcher: &MetricTypeMatcher,
    columns: HashMap<String, usize>,
) -> Metric<U>
    where
        U: Component + 'static,
        T: NetworkMetricsServiceEvent<U> + 'static + Debug,
{
    let metric_type = matcher.get_metric::<U>();
    let mut metric = Metric::<U>::new(
        event.get_shape().clone(),
        metric_type,
        columns,
        event.metric_indices()
    );
    metric
}

fn add_historical<U, T>(mut event: &T, mut metric: &mut Metric<U>)
    where
        U: Component + 'static,
        T: NetworkMetricsServiceEvent<U> + Debug + 'static
{
    let _ = get_arr_from_vec(event.get_data(), event.get_shape())
        .map(|arr| metric.historical.extend(arr, 1))
        .or_else(|e| {
            error!("Error getting shape from graph when adding metric: {:?}.", e);
            Err(e)
        });
}

fn add_metric_to_world<U>(
    commands: &mut Commands,
    metrics_lookup: &mut ResMut<MetricsState>,
    graph_dim_config: &mut ResMut<GraphConfigurationResource<U>>,
    mut graph_config: &mut ResMut<GraphingMetricsResource>,
    metric_name: &str,
    mut metric: Metric<U>,
    columns: Vec<GraphDim>,
)
    where U: Component + 'static
{

    let colors = gen_color_from_list(columns.len() as f32);

    let graph_dim_components = columns.iter()
        .enumerate()
        .map(|(i, grid_dim)| {
            let spawned_grid_component_id = commands.spawn((
                GraphDimComponent {
                    name: grid_dim.name.to_string(),
                },
                PbrBundle::default(),
                PickableBundle::default(),
                PickableComponent::GraphDim
            )).id();
            (grid_dim.name.to_string(),
             (spawned_grid_component_id, *colors.get(i).unwrap()))
        })
        .collect::<HashMap<String, (Entity, Color)>>();

    let graph_dim_entities = graph_dim_components
        .values()
        .map(|&e| e.0)
        .collect::<Vec<Entity>>();


    metric.metric_dim_component_children = graph_dim_components.clone();



// TODO: this should wait and the series is created based on changes in menu
    let metric_id = commands.spawn((
        metric,
        DataSeries {
            drawn: BTreeSet::default(),
            prev_convergence_times: Default::default()
        },
        PbrBundle::default(),
        PickableBundle::default(),
        HistoricalUpdated::default(),
        PickableComponent::Metric
    )).id();

    graph_config.metric_indices.insert(
        metric_id,
        graph_dim_components.values()
            .map(|&(e, c)| e)
            .collect()
    );

    commands.get_entity(metric_id)
        .as_mut()
        .map(|metric| {
            let graph_dim_entities = graph_dim_entities.as_slice();
            metric.push_children(graph_dim_entities);
        });

    metrics_lookup.entities.insert(metric_name.to_string(), (metric_id, 0));

    graph_dim_config.series_dims.insert(metric_id, columns);

}

fn get_graph_dims<U>(config_properties: &Res<ConfigurationProperties>, metric: &mut Metric<U>) -> Vec<GraphDim> where U: Component + 'static {
// TODO: metric type
    let columns = metric.historical.labels.iter()
        .map(|(name, index)| GraphDim {
            dim_type: vec![config_properties.metrics.get_dim_type(name.as_str())],
            name: name.clone(),
            grid_axis: config_properties.metrics.get_grid_axis(name.as_str()),
            index: *index,
        })
        .collect::<Vec<GraphDim>>();
    columns
}

fn extend_historical<T, U>(mut commands: &mut Commands, mut event: &T, timestep: &u64, entity: &mut Entity, metric: &mut Mut<Metric<U>>)
    where
        T: NetworkMetricsServiceEvent<U> + 'static + Debug,
        U: Component + 'static
{
    info!("Extending network: {:?}.", &event);
    let _ = get_arr_from_vec(event.get_data(), event.get_shape())
        .map(|arr| metric.historical.extend(arr, timestep + 1))
        .or_else(|e| {
            error!("Could not get array from vector: {:?}.", e);
            Err(e)
        });
    let _ = commands.get_entity(*entity)
        .as_mut()
        .map(|c| c.insert(HistoricalUpdated::default()));
}