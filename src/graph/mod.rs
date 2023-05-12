use std::collections::{BTreeSet, HashMap};
use std::marker::PhantomData;
use std::path::Path;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::PickableBundle;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::prelude::{Fill, Stroke};
use bevy_prototype_lyon::shapes;
use ndarray::{Array, Array1, Array2, ArrayBase, Ix1, OwnedRepr};
use serde::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, Normal};
use crate::lines::line_list::{create_3d_line, LineList, LineMaterial};
use crate::metrics::network_metrics::Metric;

pub(crate) mod setup_graph;
pub(crate) mod draw_graph_points;
pub(crate) mod graph_plugin;
pub(crate) mod radial;
pub(crate) mod graph_menu;
pub(crate) mod graph_data_event_reader;


pub const GRID_SIZE: f32 = 10000.0;
pub const GRID_LINES_THICKNESS: f32 = 0.1;
pub const GRID_AXES_THICKNESS: f32 = 0.5;
pub const GRID_COUNT: usize = 1000;
pub const NUM_GRIDLINES: usize = 1000;

/// Graph Parent
/// - > GridAxis Child: for each grid axis and plain
/// - > Metric: Graph has metric, DataSeries, which are pickable
#[derive(Component, Clone, Default, Debug)]
pub struct GraphParent {
}

#[derive(Component, Clone, Default, Debug)]
pub struct Graph<T>
    where
        T: Component {
    component: PhantomData<T>
}

#[derive(Resource, Clone, Default, Debug)]
pub struct GraphConfigurationResource<T>
where
    T: Component {
    pub(crate) series_dims: HashMap<Entity, Vec<GraphDim>>,
    pub(crate) series_type: HashMap<Entity, GraphDimType>,
    res: PhantomData<T>
}

#[derive(Clone, Debug)]
pub struct Grid {
    x_axis: Entity,
    y_axis: Entity,
    z_axis: Entity
}

#[derive(Component, Default, Clone, Deserialize, Ord, PartialOrd, PartialEq, Eq, Hash, Debug)]
pub enum GraphDimType {
    /// For each metric, there can be multiple pieces of data. For instance, one column provides
    /// the location and another column could provide a classifier that would be the color. Additionally,
    /// another column could be another continuous variable that determines the size.
    #[default]
    RadialCoordinate,
    Colored,
    Sized,
    Time
}

#[derive(Debug, Clone)]
pub struct GraphDim {
    pub(crate) dim_type: Vec<GraphDimType>,
    pub(crate) name: String,
    pub(crate) grid_axis: GridAxis,
    pub(crate) index: usize
}

#[derive(Component, Debug)]
pub struct DataSeries {
    pub(crate) drawn: BTreeSet<u64>,
    pub(crate) prev_convergence_times: HashMap<String, Vec<Option<f32>>>
}

#[derive(Component, Debug)]
pub struct SeriesStep {
}

#[derive(Component, Debug, Default, Clone, Deserialize, Serialize, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum GridAxis {
    XGridY, XGridZ,
    YGridX, YGridZ,
    ZGridX, ZGridY,
    #[default] Y, X, Z
}

