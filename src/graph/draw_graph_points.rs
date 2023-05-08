use std::default::default;
use bevy::prelude::{Assets, Changed, Children, Commands, Component, Entity, info, MaterialMeshBundle, Mesh, Query, ResMut, Vec3, With, Without};
use bevy::utils::{HashMap, HashSet};
use ndarray::SliceInfoElem;
use crate::graph::{DataSeries, Graph, SeriesStep};
use crate::lines::line_list::{create_3d_line, LineList, LineMaterial};
use crate::metrics::network_metrics::Metric;

pub(crate) fn draw_graph_points<T>(
    mut commands: Commands,
    mut graph: Query<(Entity, &mut Graph<T>, &Children), (With<Children>, Without<DataSeries>)>,
    mut metrics: Query<(Entity, &Metric<T>, &mut DataSeries), (Changed<Metric<T>>)>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<LineMaterial>>,
)
where
    T: Component
{
    let graph_children = graph.iter()
        .flat_map(|g| g.2.iter().map(move |child| (g.0, *child)))
        .collect::<HashMap<Entity, Entity>>();

    for (metric_entity, metric, series) in metrics.iter_mut() {
        let parent_entity = graph_children.get(&metric_entity).unwrap();
        series.drawn.last()
            .map(|last| metric.historical.timestep.iter()
                .skip_while(|(key,val)| **key != *last)
                .map(|(key,val)| {
                    series.columns.iter().map(|s| {
                        let out = metric.historical.retrieve_values(&s.name, *key);
                    })
                })
            );
    }
    info!("Component changed.")
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

    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(line_mesh.0),
            material: materials.add(line_mesh.1),
            ..default()
        }
    ))
        .id()

}
