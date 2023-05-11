use std::collections::HashMap;
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
use statrs::distribution::{ContinuousCDF, Normal};
use crate::lines::line_list::{create_3d_line, LineList, LineMaterial};

pub(crate) mod setup_graph;
pub(crate) mod draw_graph_points;
pub(crate) mod graph_plugin;
pub(crate) mod radial;
pub(crate) mod graph_menu;


pub const GRID_SIZE: f32 = 10000.0;
pub const GRID_LINES_THICKNESS: f32 = 0.1;
pub const GRID_AXES_THICKNESS: f32 = 0.5;
pub const GRID_COUNT: usize = 1000;
pub const NUM_GRIDLINES: usize = 1000;

/// Graph Parent
/// - > GridAxis Child: for each grid axis and plain
/// - > Metric: Graph has metric, DataSeries, which are pickable
#[derive(Component, Clone, Default, Debug)]
pub struct Graph<T>
    where
        T: Component {
    component: PhantomData<T>
}

#[derive(Component, Clone, Default, Debug)]
pub struct GraphConfigurationResource {
}


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

#[derive(Component, Default, Clone)]
pub enum GraphDimType {
    /// For each metric, there can be multiple pieces of data. For instance, one column provides
    /// the location and another column could provide a classifier that would be the color. Additionally,
    /// another column could be another continuous variable that determines the size.
    #[default]
    Coordinate,
    Colored,
    Sized,
    Time
}

pub struct GraphDim {
    pub(crate) dim_type: GraphDimType,
    pub(crate) name: String,
    pub(crate) grid_axis: GridAxis,
    pub(crate) index: usize
}

#[derive(Component)]
pub struct DataSeries {
    pub(crate) drawn: Vec<u64>,
    pub(crate) prev_convergence_times: HashMap<String, Vec<Option<f32>>>,
    pub(crate) columns: Vec<GraphDim>
}

#[derive(Component)]
pub struct SeriesStep {
}

#[derive(Component)]
pub enum GridAxis {
    XGridY, XGridZ,
    YGridX, YGridZ,
    ZGridX, ZGridY,
    X, Y, Z
}

/// Returns the convergence times for the historical values. The returned convergence times are
/// scaled between 0.0 and 1.0, where 1.0 is the estimated time of convergence.
pub(crate) fn calculate_convergence_time(
    current_time: &Vec<Option<f32>>,
    historical: Vec<ArrayBase<OwnedRepr<f32>, Ix1>>
) -> Vec<Option<f32>> {
    historical.iter().enumerate()
        .map(|(i, value)| {
            let (first_deriv, second_deriv) = calculate_derivatives(value);
            estimate_convergence_time(
                current_time.get(i)
                    .or(Some(&None))
                    .unwrap(),
                value,
                &first_deriv,
                &second_deriv,
                0.95,
                0.88
            )
        })
        .collect()
}

pub(crate) fn calculate_derivatives(loss_values: &Array1<f32>) -> (Array1<f32>, Array1<f32>) {
    let n = loss_values.len();
    let mut first_derivative = Array1::zeros(n - 1);
    let mut second_derivative = Array1::zeros(n - 2);

    // Calculate first derivative
    for i in 1..n {
        first_derivative[i - 1] = loss_values[i] - loss_values[i - 1];
    }

    // Calculate second derivative
    for i in 1..n - 1 {
        second_derivative[i - 1] = first_derivative[i] - first_derivative[i - 1];
    }

    (first_derivative, second_derivative)
}

pub(crate) fn estimate_convergence_time(
    current_time: &Option<f32>,
    loss_values: &Array1<f32>,
    first_derivative: &Array1<f32>,
    second_derivative: &Array1<f32>,
    convergence_threshold: f32,
    ema_alpha: f32,
) -> Option<f32> {

    let current_time = current_time.or(Some(0.0))
        .unwrap();

    let n = first_derivative.len();

    // Calculate the exponential moving average of the first derivatives
    let mut ema_first_derivative = first_derivative[0];
    for i in 1..n {
        //TODO: generic over # of deriv and # of moments of  std dev
        ema_first_derivative = ema_alpha * first_derivative[i] + (1.0 - ema_alpha) * ema_first_derivative;
    }

    let max_loss = loss_values.fold(f32::MIN, |v, v2| v.max(*v2));
    ema_first_derivative = ema_first_derivative / max_loss;

    // Calculate the estimated convergence time based on the exponential moving average of the first derivatives
    // and the probability of change
    let latest_loss = loss_values[loss_values.len() - 1];
    let normalized_remaining_loss = (latest_loss - convergence_threshold) / (max_loss - convergence_threshold);

    let convergence_position = get_convergence_pos(second_derivative, normalized_remaining_loss);

    // Adjust the convergence time estimate based on the probability of change
    let time_estimate = ((convergence_position / n as f32) + current_time)
        .max(0.0)
        .min(1.0);

    Some(time_estimate)
}

fn get_convergence_pos(second_derivative: &Array1<f32>, normalized_remaining_loss: f32) -> f32 {
    let mut convergence_position = normalized_remaining_loss;

    // Calculate the z-score
    let mean_second_derivative = second_derivative.mean();
    if mean_second_derivative.is_some() {
        let std_second_derivative = second_derivative.std(1.0);
        let z_score = (std_second_derivative - mean_second_derivative.unwrap()) / std_second_derivative;
        // Calculate the change probability using the CDF of the standard normal distribution
        let standard_normal = Normal::new(0.0, 1.0).unwrap();
        let change_probability = standard_normal.cdf(z_score as f64) as f32;
        convergence_position = normalized_remaining_loss * (1.0 + change_probability);
    }
    convergence_position
}

