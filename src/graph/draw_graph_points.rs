use std::collections::HashMap;
use std::f32::consts::PI;
use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::log::error;
use bevy::pbr::Material;
use bevy::prelude::{Added, Assets, BuildChildren, Changed, Children, Color, Commands, Component, default, Entity, info, MaterialMeshBundle, Mesh, Mut, Or, Parent, Query, ResMut, Vec3, With, Without};
use bevy_mod_picking::PickableBundle;
use bevy_polyline::prelude::{Polyline, PolylineMaterial};
use ndarray::{Array1, s, SliceInfoElem};
use crate::graph::{DataSeries, Graph, GraphConfigurationResource, GraphDim, GraphDimComponent, GraphDimType, GraphParent, SeriesStep};
use crate::graph::graph_data_event_reader::HistoricalUpdated;
use crate::graph::radial::calculate_radial_time;
use crate::lines::line_list::{create_3d_line, LineList};
use crate::metrics::network_metrics::{HistoricalData, Metric};

pub trait GraphingStrategy<T>
    where
        T: Component + Send + Sync + 'static,
{
    fn create_update_graph(
        commands: &mut Commands,
        metric: &Metric<T>,
        series: &mut Mut<DataSeries>,
        columns: &mut Vec<GraphDim>,
        meshes: &mut ResMut<Assets<Mesh>>,
        polylines: &mut ResMut<Assets<Polyline>>,
        materials: &mut ResMut<Assets<PolylineMaterial>>,
        num_col: usize,
        key: &u64
    );

}

fn matches(graph_dim_type: &GraphDimType) -> bool {
    match graph_dim_type {
        GraphDimType::RadialCoordinate => true,
        _ => false
    }
}

type WithDataSeriesChangedHistorical = (With<DataSeries>, Or<(Changed<HistoricalUpdated>, Added<HistoricalUpdated>)>);

/// TODO: The layer should be wide, and then the node losses should be set inside of them. So the
///     layer will be from radians a to b, and the width of the line will be the size of this, and
///     then each node will take some n fraction. where ((a - b) - epsilon_edge) / n = size_fraction
pub(crate) fn draw_graph_points<T, P>(
    mut commands: Commands,
    mut metrics: Query<
        (Entity, &Metric<T>, &mut DataSeries),
        WithDataSeriesChangedHistorical
    >,
    mut metric_dims: Query<(Entity, &GraphDimComponent)>,
    mut dims: ResMut<GraphConfigurationResource<T>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut polylines: ResMut<Assets<Polyline>>,
    mut materials: ResMut<Assets<PolylineMaterial>>,
)
where
    T: Component + Send + Sync + 'static + Debug,
    P: GraphingStrategy<T>,
{

    for (metric_entity, metric, mut series) in metrics.iter_mut() {
        let last = *series.drawn
            .last().or(Some(&1)).unwrap();
        if !dims.series_dims.contains_key(&metric_entity) {
            continue;
        }
        let num_col = dims.series_dims.get(&metric_entity).unwrap().len();
        let added = metric.historical.timestep.iter()
            .skip_while(|(key, val)| **key != last)
            .flat_map(|(key, val)| {
                if *key != last {

                    P::create_update_graph(
                        &mut commands,
                        metric,
                        &mut series,
                        &mut dims.series_dims.get_mut(&metric_entity).unwrap(),
                        &mut meshes,
                        &mut polylines,
                        &mut materials,
                        num_col,
                        key
                    );

                    vec![*key]

                }
                else {
                    vec![]
                }
            })
            .collect::<Vec<u64>>();

        series.drawn.extend(added);

    }
}

pub(crate) fn create_data_segment(
    commands: &mut Commands,
    start: Vec3,
    end: Vec3,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut polylines: &mut ResMut<Assets<Polyline>>,
    mut polyline_materials: &mut ResMut<Assets<PolylineMaterial>>,
    thickness: f32
) -> Entity {

    let line_bundler = create_3d_line(LineList {
        color: Color::GREEN,
        lines: vec![(start, end)],
        thickness,
    }, polylines, polyline_materials);

    commands
        .spawn((
            line_bundler,
            SeriesStep {}
        ))
        .id()

}
