use bevy::prelude::{Assets, Color, Commands, Component, Entity, info, Mesh, Mut, ResMut};
use bevy::math::Vec3;
use std::f32::consts::PI;
use std::collections::HashMap;
use bevy::pbr::Material;
use ndarray::{Array1, ArrayBase, Ix1, OwnedRepr};
use statrs::distribution::{ContinuousCDF, Normal};
use crate::graph;
use crate::graph::{DataSeries, GraphConfigurationResource, GraphDim, GraphDimType};
use crate::graph::draw_graph_points::{create_data_segment, GraphingStrategy};
use crate::lines::line_list::LineMaterial;
use crate::metrics::network_metrics::Metric;

pub struct RadialGraphPoints;

impl<T> GraphingStrategy<T, LineMaterial> for RadialGraphPoints
    where
        T: Component + Send + Sync + 'static
    {
        fn create_update_graph(
            mut commands: &mut Commands,
            metric: &Metric<T>,
            series: &mut Mut<DataSeries>,
            columns: &mut Vec<GraphDim>,
            mut meshes: &mut ResMut<Assets<Mesh>>,
            mut materials: &mut ResMut<Assets<LineMaterial>>,
            num_col: usize,
            key: &u64
        ) -> Vec<Entity> {
            let angle_increment = 2.0 * PI / num_col as f32;
            let radius = 100.0;
            let origin_height = 100.0;
            let convergence = &series.prev_convergence_times;

            let mut points = vec![];

            info!("Creating graph points from series: {:?} with {} number of columns and {} as key and {:?} is previous convergence.", &series, num_col, key, convergence);


            let convergence_times = columns
                .iter_mut()
                .enumerate()
                .filter(|(index, dim_type)| dim_type.dim_type
                    .iter()
                    .any(|dim_type| matches!(dim_type, GraphDimType::RadialCoordinate))
                )
                .map(|(i, graph_dim)| {

                    info!("{} is graph dim name.", graph_dim.name);

                    let starting_ending = metric.historical
                        .retrieve_values(
                            &graph_dim.name,
                            *key,
                        );

                    let (starting_values, ending_values) = starting_ending
                        .map(|(starting, ending)| {
                            (Some(starting), Some(ending))
                        })
                        .or(Some((None, None)))
                        .unwrap();


                    let (mut current_convergence_times, default, mut convergence_times)
                        = get_convergence_times(metric, &convergence, &graph_dim, &starting_values, &ending_values);

                    info!("{:?} are the current convergent times.", current_convergence_times);
                    info!("{:?} are the convergent times.", convergence_times);
                    info!("{:?} are starting and {:?} are ending.", starting_values, ending_values);

                    if starting_values.is_some() {
                        let first = starting_values.unwrap();
                        let second = ending_values.unwrap();

                        let angle = i as f32 * angle_increment;

                        let sin = angle.sin();

                        for i in 0..first.len() {

                            let (start_x, end_x) = Self::get_start_end_times(
                                current_convergence_times
                                    .or(default.as_ref())
                                    .unwrap(),
                                &mut convergence_times,
                                i
                            );

                            info!("{} is start and {} is end for radial time.", start_x, end_x);

                            let start = first[i];
                            let end = second[i];

                            let start = Vec3::new(1.0 - start_x, origin_height - start, sin * radius);
                            let end = Vec3::new(1.0 - end_x, origin_height - end, sin * radius);

                            points.push((start, end, graph_dim.dim_type.clone()))

                        }

                    }

                    (graph_dim.name.clone(), convergence_times)
                })
                .collect::<HashMap<String, Vec<Option<f32>>>>();

            series.prev_convergence_times = convergence_times;

            points.into_iter()
                .map(|(start, end, graph_dim_type)| {
                    create_data_segment(
                        &mut commands,
                        start,
                        end,
                        &mut meshes,
                        &mut materials,
                        LineMaterial { color: Color::GREEN },
                        1.0,
                    )
                })
                .collect::<Vec<Entity>>()

        }

    }

fn get_convergence_times<'a, T>(
    metric: &Metric<T>,
    convergence: &'a HashMap<String, Vec<Option<f32>>>,
    graph_dim: &GraphDim,
    starting_values: &Option<ArrayBase<OwnedRepr<f32>, Ix1>>,
    ending_values: &Option<ArrayBase<OwnedRepr<f32>, Ix1>>
) -> (Option<&'a Vec<Option<f32>>>, Option<Vec<Option<f32>>>, Vec<Option<f32>>)
where
    T: Component + Send + Sync + 'static
{
    let history = metric.historical.retrieve_historical_1d(&graph_dim.name);
    info!("Creating graph points. {:?} are starting values and {:?} are ending values.", starting_values, ending_values);
    info!("Creating graph points. {:?} is the history.", &history);

    let empty_items = vec![None; history.len()];

    let mut current_convergence_times = convergence.get(&graph_dim.name);

    let mut convergence_times;

    if current_convergence_times.is_some() {
        convergence_times = calculate_radial_time(
            current_convergence_times.unwrap(),
            history,
        );
    } else {
        convergence_times = calculate_radial_time(
            &empty_items,
            history,
        );
    }

    (current_convergence_times, Some(empty_items), convergence_times)
}

impl RadialGraphPoints {
    fn get_start_end_times(mut current_convergence_times: &Vec<Option<f32>>, mut convergence_times: &mut Vec<Option<f32>>, i: usize) -> (f32, f32) {
        let start_x = current_convergence_times.get(i)
            .map(|c| c
                .or(*convergence_times.get(i)
                    .or(Some(&Some(0.0))).unwrap())
            )
            .or(Some(Some(0.0)))
            .unwrap()
            .unwrap();
        let end_x = convergence_times.get(i)
            .or(Some(&Some(0.0)))
            .unwrap()
            .unwrap();
        (start_x, end_x)
    }
}

/// Returns the convergence times for the historical values. The returned convergence times are
/// scaled between 0.0 and 1.0, where 1.0 is the estimated time of convergence.
pub(crate) fn calculate_radial_time(
    current_time: &Vec<Option<f32>>,
    historical: Vec<ArrayBase<OwnedRepr<f32>, Ix1>>
) -> Vec<Option<f32>> {
    historical.iter().enumerate()
        .map(|(i, value)| {
            let (first_deriv, second_deriv) = calculate_derivatives(value);
            estimate_radial_time(
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

pub(crate) fn estimate_radial_time(
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
