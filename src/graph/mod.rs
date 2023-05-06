use std::path::Path;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::PickableBundle;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::prelude::{Fill, Stroke};
use bevy_prototype_lyon::shapes;
use ndarray::{Array, Array2};
use crate::lines::line_list::{create_3d_line, LineList, LineMaterial};

pub(crate) mod setup_graph;
pub(crate) mod draw_graph_points;
pub(crate) mod graph_plugin;


pub const GRID_SIZE: f32 = 10000.0;
pub const GRID_LINES_THICKNESS: f32 = 0.1;
pub const GRID_AXES_THICKNESS: f32 = 0.5;
pub const GRID_COUNT: usize = 1000;
pub const NUM_GRIDLINES: usize = 1000;

#[derive(Resource, Default)]
struct GraphData {
    series: String,
    convergence_data: Array2<f32>,
}

#[derive(Clone, Debug)]
pub struct Grid {
    x_axis: Entity,
    y_axis: Entity,
    z_axis: Entity
}

#[derive(Component, Clone, Default, Debug)]
pub struct Graph {
}

#[derive(Component)]
pub struct LossConvergenceSeries {
}

#[derive(Component)]
pub struct LossConvergenceStep {
}

#[derive(Component)]
pub enum GridAxis {
    XGridY, XGridZ, YGridX, YGridZ, ZGridX, ZGridY, X, Y, Z
}
