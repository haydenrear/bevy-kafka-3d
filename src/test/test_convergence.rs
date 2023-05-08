use ndarray::array;
use crate::graph::{calculate_convergence_time, calculate_derivatives, estimate_convergence_time};
use super::*;

#[test]
fn test_estimate_convergence_time_exp() {
    let current_time = Some(0.5);
    let loss_values = array![1.0, 0.8, 0.7, 0.65, 0.64, 0.63, 0.62];
    let (first_derivative, second_derivative) = calculate_derivatives(&loss_values);

    let convergence_time = estimate_convergence_time(
        &current_time,
        &loss_values,
        &first_derivative,
        &second_derivative,
        0.05,
        0.9,
    );

    assert!(convergence_time.is_some());
    println!("{:?} is convergence_time.", convergence_time);
}

#[test]
fn test_estimate_convergence_time() {
    let current_time = Some(0.5);
    let loss_values = array![1.0, 0.8, 0.6, 0.4, 0.2];
    let first_derivative = array![-0.2, -0.2, -0.2];
    let second_derivative = array![0.0, 0.0];

    let convergence_time = estimate_convergence_time(
        &current_time,
        &loss_values,
        &first_derivative,
        &second_derivative,
        0.05,
        0.9,
    );

    assert!(convergence_time.is_some());
    println!("{:?} is convergence_time.", convergence_time);
}

#[test]
fn test_calculate_convergence_time() {
    let current_time = vec![Some(0.0), Some(0.0)];
    let historical = vec![
        array![1.0, 0.8, 0.6, 0.4],
        array![2.0, 1.6, 1.2, 0.8],
    ];

    let convergence_time = calculate_convergence_time(&current_time, historical);

    assert_eq!(convergence_time.len(), 2);

    assert!(convergence_time[0].is_some());
    println!("{:?} is convergence_time.", convergence_time[0]);

    assert!(convergence_time[1].is_some());
    println!("{:?} is convergence_time.", convergence_time[1]);
}
