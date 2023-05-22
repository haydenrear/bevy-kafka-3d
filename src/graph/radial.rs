use bevy::prelude::{Assets, Color, Commands, Component, Entity, error, info, Mesh, Mut, ResMut};
use bevy::math::Vec3;
use std::f32::consts::PI;
use std::collections::HashMap;
use bevy::pbr::Material;
use ndarray::{Array, Array1, Array2, ArrayBase, Axis, Ix1, OwnedRepr, s};
use ndarray_stats::{CorrelationExt, SummaryStatisticsExt};
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
        key: &u64,
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

                let graph_dim_name = &graph_dim.name;
                let starting_ending = metric.historical
                    .retrieve_values(
                        graph_dim_name,
                        *key,
                    );

                let (starting_values, ending_values) = starting_ending
                    .map(|(starting, ending)| {
                        (Some(starting), Some(ending))
                    })
                    .or(Some((None, None)))
                    .unwrap();


                let (mut current_convergence_times, default, mut convergence_times)
                    = get_convergence_times::<T>(&starting_values, &ending_values, convergence.get(graph_dim_name), metric.historical.retrieve_historical_1d(graph_dim_name));

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
                            i,
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

pub enum InterpolationOptions {
    NthMomentMthDerivative {
        n_moments: usize,
        m_derivatives: usize
    }
}

pub trait Interpolator {
    fn get_next_value(
        values: &ArrayBase<OwnedRepr<f32>, Ix1>,
        interpolation_options: InterpolationOptions
    ) -> Option<f32>;
}

fn get_convergence_times<'a, T>(
    starting_values: &Option<ArrayBase<OwnedRepr<f32>, Ix1>>,
    ending_values: &Option<ArrayBase<OwnedRepr<f32>, Ix1>>,
    current_convergence: Option<&'a Vec<Option<f32>>>,
    history: Vec<ArrayBase<OwnedRepr<f32>, Ix1>>,
) -> (Option<&'a Vec<Option<f32>>>, Option<Vec<Option<f32>>>, Vec<Option<f32>>)
    where
        T: Component + Send + Sync + 'static
{
    info!("Creating graph points. {:?} are starting values and {:?} are ending values.", starting_values, ending_values);
    info!("Creating graph points. {:?} is the history.", &history);

    let empty_items = vec![None; history.len()];

    let mut current_convergence_times = current_convergence;

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
    historical: Vec<ArrayBase<OwnedRepr<f32>, Ix1>>,
) -> Vec<Option<f32>> {
    historical.iter().enumerate()
        .map(|(i, value)| {
            let derivs = calculate_derivatives(value, 2);
            let first_deriv = derivs.get(0).unwrap();
            let second_deriv = derivs.get(1).unwrap();
            estimate_radial_time(
                current_time.get(i)
                    .or(Some(&None))
                    .unwrap(),
                value,
                &first_deriv,
                &second_deriv,
                0.95,
                0.88,
            )
        })
        .collect()
}

pub(crate) fn calculate_derivatives(loss_values: &Array1<f32>, n_derivatives: usize) -> Vec<Array1<f32>> {

    if n_derivatives <= 0 {
        return vec![];
    }

    let n = loss_values.len();

    let mut derivatives: Vec<Array1<f32>> = vec![];

    for i in 0..n_derivatives  {
        let next_deriv_size = n - (i + 1);

        if next_deriv_size <= 0 {
            error!("Derivative size {} and number of derivatives {} not valid and produced negative derivative size.", next_deriv_size, n_derivatives);
            return derivatives;
        }

        let mut next_deriv = Array1::zeros(next_deriv_size);

        if i == 0 {
            for j in 1..n  {
                next_deriv[j - 1] = loss_values[j] - loss_values[j - 1];
            }
        } else {
            for j in 1..n - i  {
                next_deriv[j - 1] = derivatives[i - 1][j] - derivatives[i - 1][j - 1];
            }
        }

        derivatives.push(next_deriv);
    }

    derivatives
}

pub(crate) fn calculate_moments(inputs: &Array1<f32>, m_moments: usize) -> Vec<Array1<f32>> {
    let n = inputs.len();

    if n < 2 {
        error!("Attempted to calculate moments from too small of a series.");
        return vec![];
    }

    let mut moments: Vec<Array1<f32>> = vec![];
    for i in 0..m_moments + 1 {
        moments.push(Array1::from_vec(vec![0.0; n]))
    }

    for i in 1..n + 1 {
        let from_slice = inputs.slice(s![..i]);
        let moments_created = from_slice
            .central_moments(m_moments as u16);
        let std_dev = from_slice.std(1.0);
        let moments_vec = moments_created.unwrap();
        for (idx, moment_found) in moments_vec.iter().enumerate() {
            let std_pow = std_dev.powf((idx + 1) as f32);

            if std_pow == 0.0 || *moment_found == f32::NAN {
                moments[idx][i-1] = 0.0;
            }

            if idx == 0 {
                moments[idx][i - 1] = from_slice.mean().or(Some(0.0)).unwrap() / std_pow;
            } else if idx == 1 {
                moments[idx][i - 1] = from_slice.var(1.0) / std_pow;
            } else {
                let moment_created = *moment_found / std_pow;
                if moment_created == f32::NAN {
                    moments[idx][i - 1] = 0.0;
                } else {
                    moments[idx][i - 1] = moment_created;
                }
            }
        }
    }


    for (size, moment) in moments.iter().enumerate() {
        println!("{:?} is the {}nth moment", moment.as_slice().unwrap(), size);
    }

    moments
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

    // you take some derivative, and you calculate the moment for each timestep, with all of the previous,
    // - then you have three sequences, the derivative that you calculated, and the moment that you calculated
    //   from it, and then the original value that the derivative was calculated from.
    // So each of the moments is sort of like a gauge for the previous. Like how reliant is the previous.
    //    maybe the derivative is analogous to the moment, and the second derivative can use the second moment
    //    to normalize, etc, and then at each successive level you have one more chance to catch the moment.
    //  and then if the moment is zero then we just add the whole previous moment

    // So we take some n derivatives and some m rolling moments where you calculate the next moment based
    // on the sequence. Then you have divide the derivative by it's associated moment and you get the
    // normalized change, and you have these normalized changes for each derivative, and you can compare
    // these between.

    // Then you take your normalized changes for each moment and you can see the change in moment for
    // change in sequence. And you're looking for when it converges to 0, or when the sequence doesn't
    // change in the moment anymore. For instance this is like when the variance is zero, like adding
    // tails successively, adding the mean tails, then the tails of that,  then the tails of that, etc.
    // until you've added all the tails.
    let latest_loss = loss_values[loss_values.len() - 1];
    let normalized_remaining_loss = (latest_loss - convergence_threshold) / (max_loss - convergence_threshold);

    let convergence_position = get_convergence_pos(second_derivative, normalized_remaining_loss);

    // Adjust the convergence time estimate based on the probability of change
    let time_estimate = ((convergence_position / n as f32) + current_time)
        .max(0.0)
        .min(1.0);

    Some(time_estimate)
}

fn get_convergence_pos(
    second_derivative: &Array1<f32>,
    normalized_remaining_loss: f32
) -> f32 {
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
