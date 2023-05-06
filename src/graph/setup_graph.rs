use bevy::prelude::{Color, Commands, default, Entity, Mesh, ResMut};
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::pbr::{MaterialMeshBundle, PbrBundle};
use bevy::hierarchy::BuildChildren;
use crate::graph::{Graph, Grid, GRID_AXES_THICKNESS, GRID_LINES_THICKNESS, GRID_SIZE, GridAxis, LossConvergenceSeries, NUM_GRIDLINES};
use crate::lines::line_list::{create_3d_line, LineList, LineMaterial};

pub(crate) fn setup_graph(mut commands: Commands,
                          mut meshes: ResMut<Assets<Mesh>>,
                          mut materials: ResMut<Assets<LineMaterial>>
) {

    let grid = draw_axes(&mut commands, &mut materials, &mut meshes, GRID_SIZE);
    draw_gridlines(&mut commands, &mut materials, &mut meshes, GRID_SIZE, &grid);
    let loss_convergence_series = commands.spawn((LossConvergenceSeries {}, PbrBundle::default())).id();
    let mut graph_component = commands.spawn((Graph {}, PbrBundle::default()));
    graph_component.add_child(loss_convergence_series);
    graph_component.add_child(grid.x_axis);
    graph_component.add_child(grid.y_axis);
    graph_component.add_child(grid.z_axis);
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
