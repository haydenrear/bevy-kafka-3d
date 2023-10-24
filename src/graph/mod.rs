use std::collections::{BTreeSet, HashMap, HashSet};
use std::marker::PhantomData;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF};
use crate::data_subscriber::metric_event::MetricComponentType;

pub(crate) mod setup_graph;
/// The data is inserted with T: Component as Node, Layer, etc. based on the topic name. The python
/// program loads the gradients from the CubeFS file store (volume), calculates the metrics, and then sends them
/// to that topic. The kafka topics matching the pattern are included.
/// However, there is only metrics calculated, and then they will be graphed, and only radial graph. In the metrics
/// data, an index needs to be included, and this index will determine how to "zero in" on data. For example,
/// when you are graphing a particular metric, it is a part of a particular layer, and the the layer is part
/// of a network. So let's say you have the loss graphed, and you have a line for each of the layers, and then
/// you hover over one of the lines of the loss, and you want to see related metrics. So you click it, and
/// a menu pops up with related metrics to add. So you index by layer, node, network for this purpose.
pub(crate) mod draw_graph_points;
pub(crate) mod graph_plugin;
pub(crate) mod radial;
pub(crate) mod graph_data_event_reader;


pub const GRID_SIZE: f32 = 100.0;
pub const GRID_LINES_THICKNESS: f32 = 0.1;
pub const GRID_AXES_THICKNESS: f32 = 1.0;
pub const GRID_COUNT: usize = 100;
pub const NUM_GRIDLINES: usize = 100;

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
    T: Component
{
    pub(crate) series_dims: HashMap<Entity, Vec<GraphDim>>,
    res: PhantomData<T>
}

#[derive(Resource, Clone, Default, Debug)]
pub struct GraphingMetricsResource
{
    /// when a metric is picked, events will be sent to be read for the entities associated with
    /// the same entities, to be read by an event reader of type T: Component, such as Node, Layer, etc.
    pub(crate) graphing_indices: HashMap<String, HashSet<Entity>>,
    pub(crate) metric_indices: HashMap<Entity, HashSet<Entity>>,
    pub(crate) index_types: HashMap<MetricComponentType, HashSet<String>>,
    pub(crate) state_transition_completed: Vec<Entity>
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
    pub(crate) index: usize,
}

#[derive(Component)]
pub struct GraphDimComponent {
    pub(crate) name: String
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

