use std::collections::HashMap;
use std::default::default;
use std::f32::consts::PI;
use std::marker::PhantomData;
use bevy::prelude::{Assets, BuildChildren, Changed, Children, Color, Commands, Component, Entity, info, MaterialMeshBundle, Mesh, Mut, Query, ResMut, Vec3, With, Without};
use bevy_mod_picking::PickableBundle;
use ndarray::{Array1, s, SliceInfoElem};
use crate::graph::{calculate_convergence_time, DataSeries, Graph, GraphDimType, SeriesStep};
use crate::lines::line_list::{create_3d_line, LineList, LineMaterial};
use crate::metrics::network_metrics::{HistoricalData, Metric};

pub trait GraphPoints<T>
    where
        T: Component + Send + Sync + 'static
{
    fn get_graph_points(
        metric: &Metric<T>,
        series: &mut Mut<DataSeries>,
        num_col: usize,
        key: &u64
    ) -> Vec<(Vec3, Vec3, GraphDimType)>;
}

pub(crate) fn draw_graph_points<T, P>(
    mut commands: Commands,
    mut graph: Query<
        (Entity, &mut Graph<T>, &Children),
        (With<Children>, Without<DataSeries>)
    >,
    mut metrics: Query<
        (Entity, &Metric<T>, &mut DataSeries),
        (Changed<Metric<T>>, Without<Graph<T>>, With<DataSeries>)
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
)
where
    T: Component + Send + Sync + 'static,
    P: GraphPoints<T>
{
    let graph_children = graph.iter()
        .flat_map(|g| g.2.iter().map(move |child| (g.0, *child)))
        .collect::<HashMap<Entity, Entity>>();

    for (metric_entity, metric, mut series) in metrics.iter_mut() {
        let parent_metric = graph_children.get(&metric_entity).unwrap();
        let last = *series.drawn
            .last().or(Some(&0))
            .unwrap();
        let num_col = series.columns.len();
        let added = metric.historical.timestep.iter()
            .skip_while(|(key, val)| **key != last)
            .flat_map(|(key, val)| {
                if *key != last {
                    P::get_graph_points(metric, &mut series, num_col, key)
                        .into_iter()
                        .for_each(|(start, end, dim_type)| {

                            /// need to match up the layer
                            let next_segment = create_data_segment(
                                &mut commands,
                                start,
                                end,
                                &mut meshes,
                                &mut materials,
                                LineMaterial { color: Color::GREEN },
                                1.0
                            );

                            commands.get_entity(*parent_metric)
                                .as_mut()
                                .map(|parent| parent.add_child(next_segment));
                        });
                    vec![*key]
                }
                else {
                    vec![]
                }
            })
            .collect::<Vec<u64>>();

        series.drawn.extend(added);

        series.drawn.sort();


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
            PickableBundle::default()
        ))
        .id()

}
