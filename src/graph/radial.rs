use bevy::prelude::{Component, Mut};
use bevy::math::Vec3;
use std::f32::consts::PI;
use std::collections::HashMap;
use crate::graph::{calculate_convergence_time, DataSeries, GraphDimType};
use crate::graph::draw_graph_points::GraphPoints;
use crate::metrics::network_metrics::Metric;

pub struct RadialGraphPoints;

impl<T> GraphPoints<T> for RadialGraphPoints
    where
        T: Component + Send + Sync + 'static
    {
        fn get_graph_points(metric: &Metric<T>, series: &mut Mut<DataSeries>, num_col: usize, key: &u64) -> Vec<(Vec3, Vec3, GraphDimType)> {
            let angle_increment = 2.0 * PI / num_col as f32;
            let radius = 100.0;
            let origin_height = 100.0;
            let convergence = &series.prev_convergence_times;

            let mut points = vec![];

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
                            points.push((start, end, graph_dim.dim_type.clone()))

                        }
                    }

                    (graph_dim.name.clone(), convergence_times)
                })
                .collect::<HashMap<String, Vec<Option<f32>>>>();

            series.prev_convergence_times = convergence_times;

            points
        }
    }
