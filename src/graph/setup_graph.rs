use std::marker::PhantomData;
use bevy::prelude::{Added, Color, Commands, Component, default, Entity, info, Mesh, Query, ResMut, Visibility};
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::pbr::{MaterialMeshBundle, PbrBundle};
use bevy::hierarchy::BuildChildren;
use bevy::log::error;
use crate::graph::{Graph, GraphingMetricsResource, GraphParent, Grid, GRID_AXES_THICKNESS, GRID_LINES_THICKNESS, GRID_SIZE, GridAxis, NUM_GRIDLINES};
use crate::lines::line_list::{create_3d_line, LineList, LineMaterial};
use crate::menu::config_menu_event::interaction_config_event_writer::{GraphMenuResultBuilder, NetworkMenuResultBuilder};
use crate::menu::ui_menu_event::transition_groups::PropagateVisible;
use crate::metrics::network_metrics::Metric;
use crate::util;

/// When a metric is added to the world, a graph is created for this metric, which has a series.
pub(crate) fn graph_points_generator<T>
(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut graph_config: ResMut<GraphingMetricsResource>,
    metric_added_event: Query<(Entity, &Metric<T>), (Added<Metric<T>>)>,
    graph_parent_query: Query<(Entity, &GraphParent)>,
)
    where
        T: Component
{
    for (metric_entity, metric_added) in metric_added_event.iter() {
        let graph = Graph {
            component: PhantomData::<T>::default(),
        };
        add_indices(&mut graph_config, metric_entity, metric_added);
        let mut graph = commands.spawn((graph, PbrBundle::default()));
        let _ = graph_parent_query.get_single()
            .map(|(graph_parent_entity, _)| graph.set_parent(graph_parent_entity))
            .or_else(|e| {
                error!("Could not set parent for graph parent: {:?}", e);
                Err(e)
            });
        info!("Adding metric entity as child.");
        graph.add_child(metric_entity);
    }
}

fn add_indices<T>(mut graph_config: &mut ResMut<GraphingMetricsResource>, metric_entity: Entity, metric_added: &Metric<T>)
where
    T: Component
{
    metric_added.metric_indices.iter()
        .for_each(|(metric_component_type, indices)| {
            indices.iter().for_each(|metric_index| {
                util::add_or_insert(metric_component_type, metric_index.to_string(), &mut graph_config.index_types);
                util::add_or_insert(metric_index, metric_entity, &mut graph_config.graphing_indices);
            });
        });
}

pub(crate) fn setup_graph(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut context: ResMut<GraphMenuResultBuilder>,
) {
    draw_graph(&mut commands, &mut meshes, &mut materials, &mut context);
}

fn draw_graph(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<LineMaterial>>,
    context: &mut ResMut<GraphMenuResultBuilder>,
) -> Entity
{
    let grid = draw_axes(&mut commands, &mut materials, &mut meshes, GRID_SIZE);
    draw_gridlines(&mut commands, &mut materials, &mut meshes, GRID_SIZE, &grid);
    let mut graph_component = commands.spawn((
        GraphParent::default(),
        PbrBundle::default(),
        PropagateVisible::default()
    ));
    graph_component.add_child(grid.x_axis);
    graph_component.add_child(grid.y_axis);
    graph_component.add_child(grid.z_axis);
    graph_component.insert(Visibility::Visible);
    let graph = graph_component.id();
    info!("Made {:?} visible.", graph);
    context.graph_parent_entity = Some(graph);
    graph
}

fn draw_axes(
    mut commands: &mut Commands,
    mut materials: &mut ResMut<Assets<LineMaterial>>,
    mesh: &mut ResMut<Assets<Mesh>>,
    size: f32,
) -> Grid {
    create_axes(commands, materials, mesh, size * 2 as f32)
}

fn draw_gridlines(
    mut commands: &mut Commands,
    mut materials: &mut ResMut<Assets<LineMaterial>>,
    mesh: &mut ResMut<Assets<Mesh>>,
    size: f32,
    grid: &Grid
) {
    let spacing = (size * 2.0) as f32 / NUM_GRIDLINES as f32;

    for i in 0..NUM_GRIDLINES {
        let offset = i as f32 * spacing - size;
        write_x_axis(&mut commands, materials, mesh, size, offset, grid.x_axis);
        write_y_axis(&mut commands, materials, mesh, size, offset, grid.y_axis);
        write_z_axis(&mut commands, materials, mesh, size, offset, grid.z_axis);
    }
}

fn write_z_axis(mut commands: &mut &mut Commands, mut materials: &mut ResMut<Assets<LineMaterial>>, mesh: &mut ResMut<Assets<Mesh>>, size: f32, offset: f32, entity: Entity) {
    let starting_pt = Vec3::new(offset, 0.0, -size);
    let ending_pt = Vec3::new(offset, 0.0, size);
    let z_grid_x = create_grid_line(&mut commands, starting_pt, ending_pt, mesh, materials, LineMaterial {
        color: Color::GRAY,
    }, GRID_LINES_THICKNESS, GridAxis::ZGridX);
    let starting_pt = Vec3::new(0.0, offset, -size);
    let ending_pt = Vec3::new(0.0, offset, size);
    let z_grid_y = create_grid_line(&mut commands, starting_pt, ending_pt, mesh, materials, LineMaterial {
        color: Color::GRAY,
    }, GRID_LINES_THICKNESS, GridAxis::ZGridY);
    commands.get_entity(entity)
        .as_mut()
        .map(|parent| {
            parent.add_child(z_grid_y);
            parent.add_child(z_grid_x);
        });
}

fn write_y_axis(mut commands: &mut &mut Commands, mut materials: &mut ResMut<Assets<LineMaterial>>, mesh: &mut ResMut<Assets<Mesh>>, size: f32, offset: f32, entity: Entity) {
    let starting_pt = Vec3::new(0.0, -size, offset);
    let ending_pt = Vec3::new(0.0, size, offset);
    let y_grid_z = create_grid_line(&mut commands, starting_pt, ending_pt, mesh, materials, LineMaterial {
        color: Color::GRAY,
    }, GRID_LINES_THICKNESS, GridAxis::YGridZ);
    let starting_pt = Vec3::new(offset, size, 0.0);
    let ending_pt = Vec3::new(offset, -size, 0.0);
    let y_grid_x = create_grid_line(&mut commands, starting_pt, ending_pt, mesh, materials, LineMaterial {
        color: Color::GRAY,
    }, GRID_LINES_THICKNESS, GridAxis::YGridX);

    commands.get_entity(entity)
        .as_mut()
        .map(|parent| {
            parent.add_child(y_grid_x);
            parent.add_child(y_grid_z);
        });
}

fn write_x_axis(mut commands: &mut Commands, mut materials: &mut ResMut<Assets<LineMaterial>>, mesh: &mut ResMut<Assets<Mesh>>, size: f32, offset: f32, entity: Entity) {
    let starting_pt = Vec3::new(-size, 0.0, offset);
    let ending_pt = Vec3::new(size, 0.0, offset);
    let x_grid_z = create_grid_line(&mut commands, starting_pt, ending_pt, mesh, materials, LineMaterial {
        color: Color::GRAY,
    }, GRID_LINES_THICKNESS, GridAxis::XGridZ);
    let starting_pt = Vec3::new(-size, offset, 0.0);
    let ending_pt = Vec3::new(size, offset, 0.0);
    let x_grid_y = create_grid_line(&mut commands, starting_pt, ending_pt, mesh, materials, LineMaterial {
        color: Color::GRAY,
    }, GRID_LINES_THICKNESS, GridAxis::XGridY);

    commands.get_entity(entity)
        .as_mut()
        .map(|parent| {
            parent.add_child(x_grid_y);
            parent.add_child(x_grid_z);
        });
}

fn create_axes(
    commands: &mut Commands,
    mut materials: &mut ResMut<Assets<LineMaterial>>,
    mesh: &mut ResMut<Assets<Mesh>>,
    size: f32
) -> Grid {
    let axes_material = LineMaterial {
        color: Color::BLACK,
    };

    let x_axis = create_grid_line(commands, Vec3::new(-size, 0.0, 0.0), Vec3::new(size, 0.0, 0.0), mesh, materials, axes_material.clone(), GRID_AXES_THICKNESS, GridAxis::X);
    let y_axis = create_grid_line(commands, Vec3::new(0.0, -size, 0.0), Vec3::new(0.0, size, 0.0), mesh, materials, axes_material.clone(), GRID_AXES_THICKNESS, GridAxis::Y);
    let z_axis = create_grid_line(commands, Vec3::new(0.0, 0.0, -size), Vec3::new(0.0, 0.0, size), mesh, materials, axes_material.clone(), GRID_AXES_THICKNESS, GridAxis::Z);

    return Grid{
        x_axis,
        y_axis,
        z_axis,
    }
}


fn create_grid_line(
    commands: &mut Commands,
    start: Vec3,
    end: Vec3,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<LineMaterial>>,
    material: LineMaterial,
    thickness: f32,
    parent: GridAxis
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
        .insert(parent)
        .id()

}

