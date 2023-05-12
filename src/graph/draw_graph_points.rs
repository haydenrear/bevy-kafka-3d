use std::collections::HashMap;
use std::default::default;
use std::f32::consts::PI;
use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::log::error;
use bevy::pbr::Material;
use bevy::prelude::{Added, Assets, BuildChildren, Changed, Children, Color, Commands, Component, Entity, info, MaterialMeshBundle, Mesh, Mut, Or, Parent, Query, ResMut, Vec3, With, Without};
use bevy_mod_picking::PickableBundle;
use ndarray::{Array1, s, SliceInfoElem};
use crate::graph::{DataSeries, Graph, GraphConfigurationResource, GraphDim, GraphDimType, GraphParent, SeriesStep};
use crate::graph::graph_data_event_reader::HistoricalUpdated;
use crate::graph::radial::calculate_radial_time;
use crate::lines::line_list::{create_3d_line, LineList, LineMaterial};
use crate::metrics::network_metrics::{HistoricalData, Metric};

pub trait GraphingStrategy<T, M>
    where
        T: Component + Send + Sync + 'static,
        M: Material
{
    fn create_update_graph(
        commands: &mut Commands,
        metric: &Metric<T>,
        series: &mut Mut<DataSeries>,
        columns: &mut Vec<GraphDim>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<M>>,
        num_col: usize,
        key: &u64
    ) -> Vec<Entity>;

}

fn matches(graph_dim_type: &GraphDimType) -> bool {
    match graph_dim_type {
        GraphDimType::RadialCoordinate => true,
        _ => false
    }
}

type WithDataSeriesChangedHistorical = (With<DataSeries>, Or<(Changed<HistoricalUpdated>, Added<HistoricalUpdated>)>);

pub(crate) fn draw_graph_points<T, P, M>(
    mut commands: Commands,
    mut metrics: Query<
        (Entity, &Metric<T>, &mut DataSeries),
        WithDataSeriesChangedHistorical
    >,
    mut dims: ResMut<GraphConfigurationResource<T>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<M>>,
)
where
    T: Component + Send + Sync + 'static + Debug,
    P: GraphingStrategy<T, M>,
    M: Material
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

                    let next_segment = P::create_update_graph(
                        &mut commands, metric,
                        &mut series,
                        &mut dims.series_dims.get_mut(&metric_entity).unwrap(),
                        &mut meshes,
                        &mut materials,
                        num_col,
                        key
                    );

                    commands.get_entity(metric_entity)
                        .as_mut()
                        .map(|metric_parent| metric_parent.push_children(next_segment.as_slice()));

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
    mut materials: &mut ResMut<Assets<LineMaterial>>,
    material: LineMaterial,
    thickness: f32
) -> Entity {

    let line_mesh = create_3d_line(LineList {
        lines: vec![(start, end)],
        thickness,
    }, material);

    commands
        .spawn((
            MaterialMeshBundle {
                mesh: meshes.add(line_mesh.0),
                material: materials.add(line_mesh.1),
                ..default()
            },
            PickableBundle::default(),
            SeriesStep {}
        ))
        .id()

}
