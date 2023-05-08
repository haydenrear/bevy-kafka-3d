use std::collections::HashMap;
use std::default::default;
use std::f32::consts::PI;
use std::marker::PhantomData;
use bevy::prelude::{Assets, BuildChildren, Changed, Children, Color, Commands, Component, Entity, info, MaterialMeshBundle, Mesh, Query, ResMut, Vec3, With, Without};
use ndarray::{Array1, s, SliceInfoElem};
use crate::graph::{calculate_convergence_time, DataSeries, Graph, GraphDimType, SeriesStep};
use crate::lines::line_list::{create_3d_line, LineList, LineMaterial};
use crate::metrics::network_metrics::{HistoricalData, Metric};

pub(crate) fn draw_graph_points<T>(
    mut commands: Commands,
    mut graph: Query<
        (Entity, &mut Graph<T>, &Children),
        (With<Children>, Without<DataSeries>)
    >,
    mut metrics: Query<
        (Entity, &Metric<T>, &mut DataSeries),
        (Changed<Metric<T>>, Without<Graph<T>>)
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
)
where
    T: Component + Send + Sync + 'static
{
    let graph_children = graph.iter()
        .flat_map(|g| g.2.iter().map(move |child| (g.0, *child)))
        .collect::<HashMap<Entity, Entity>>();

    for (metric_entity, metric, mut series) in metrics.iter_mut() {
        let parent_entity = graph_children.get(&metric_entity).unwrap();
        let last = *series.drawn.last().or(Some(&0)).unwrap();
        let num_col = series.columns.len();
        let _ = metric.historical.timestep.iter()
            .skip_while(|(key, val)| **key != last)
            .map(|(key, val)| {
                let angle_increment = 2.0 * PI / num_col as f32;
                let radius = 100.0;
                let origin_height = 100.0;
                let convergence = &series.prev_convergence_times;
                let convergence_times = series.columns
                    .iter()
                    .enumerate()
                    .map(|(i, graph_dim)| {
                        let (starting_values, ending_values) = metric.historical
                            .retrieve_values(
                                &graph_dim.name,
                                *key,
                            );

                        let history = metric.historical.retrieve_historical_1d(&graph_dim.name);

                        let empty_items = vec![None; num_col];
                        let mut current_convergence_times = convergence.get(&graph_dim.name)
                            .or(Some(&empty_items))
                            .unwrap();

                        let convergence_times = calculate_convergence_time(
                            current_convergence_times,
                            history,
                        );

                        if starting_values.is_some() {

                            let first = starting_values.unwrap();
                            let second = ending_values.unwrap();

                            let angle = i as f32 * angle_increment;

                            let sin = angle.sin();

                            for i in 0..first.len() {
                                let start = first[i];
                                let end = second[i];
                                let start_x = current_convergence_times.get(i)
                                    .map(|c| c
                                        .or(*convergence_times.get(i).or(Some(&None)).unwrap())
                                    )
                                    .or(Some(None))
                                    .unwrap()
                                    .unwrap();
                                let end_x = convergence_times.get(i)
                                    .or(Some(&None))
                                    .unwrap()
                                    .unwrap();

                                let start = Vec3::new(1.0 - start_x, origin_height - start, sin * radius);
                                let end = Vec3::new(1.0 - end_x, origin_height - end, sin * radius);

                                let next_segment = create_data_segment(
                                    &mut commands,
                                    start,
                                    end,
                                    &mut meshes,
                                    &mut materials,
                                    LineMaterial { color: Color::GREEN },
                                    1.0
                                );

                                commands.get_entity(*parent_entity)
                                    .as_mut()
                                    .map(|parent| parent.add_child(next_segment));
                            }
                        }

                        (graph_dim.name.clone(), convergence_times)

                    })
                    .collect::<HashMap<String, Vec<Option<f32>>>>();

                series.prev_convergence_times = convergence_times;

            });
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

    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(line_mesh.0),
            material: materials.add(line_mesh.1),
            ..default()
        }
    ))
        .id()

}
