use approx::assert_abs_diff_eq;
use bevy::prelude::system_adapter::new;
use bevy_inspector_egui::restricted_world_view::Error::NoTypeData;
use ndarray::{array, Array, Array1, ArrayView, Axis, Ix1, OwnedRepr, s};
use ndarray_stats::{EntropyExt, QuantileExt, SummaryStatisticsExt};
use statrs::distribution::{Continuous, ContinuousCDF, Gamma, Weibull};
use statrs::statistics::Statistics;
use ndarray::prelude::*;
use statrs::function::gamma::{digamma, gamma};
use statrs::StatsError;
// use rand::prelude::Distribution;
// use linfa::traits::{Fit, Transformer};
// use linfa_preprocessing::linear_scaling::LinearScaler;
// use linfa::DatasetBase;
// use argmin::core::{CostFunction, Error, Executor, Gradient};
// use argmin::solver::gradientdescent::SteepestDescent;
// use argmin::solver::linesearch::{BacktrackingLineSearch, HagerZhangLineSearch, MoreThuenteLineSearch};
// use argmin::solver::linesearch::condition::ArmijoCondition;

use crate::graph::radial::{calculate_derivatives, calculate_moments};

#[test]
fn test_calculate_derivatives() {
    let loss_values = array![1.0, 2.0, 4.0, 7.0, 11.0];

    let derivatives = calculate_derivatives(&loss_values, 4);

    assert_eq!(derivatives.len(), 4);

    // Expected first derivatives: [1.0, 2.0, 3.0, 4.0]
    assert_eq!(derivatives[0], array![1.0, 2.0, 3.0, 4.0]);

    // Expected second derivatives: [1.0, 1.0, 1.0]
    assert_eq!(derivatives[1], array![1.0, 1.0, 1.0]);

    // Expected third derivatives: [0.0, 0.0]
    assert_eq!(derivatives[2], array![0.0, 0.0]);

    // Expected fourth derivatives: [0.0]
    assert_eq!(derivatives[3], array![0.0]);
}

#[test]
fn test_calculate_moments() {
    let arr = Array::from_vec(vec![22.0, 46.0, 78.0, 120.0, 30.0, 56.0, 78.0, 23.0]);
    let moments = calculate_moments(&arr, 5);

    assert_eq!(moments.len(), 6);
    assert!(moments.iter().all(|m| m.len() == 8));
}

// pub struct GammaOptimizer {
//     data: Array1<f32>,
//
// }
//
// impl CostFunction for GammaOptimizer {
//     type Param = ArrayBase<OwnedRepr<f32>, ndarray::Dim<[usize; 1]>>;
//     type Output = f32;
//
//     fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
//         // println!("{} is shape and {} is rate", param[0], param[1]);
//         let mut shape = param[0];
//         let mut rate = param[1];
//         if shape.is_nan() || shape == 0.0 || shape.is_infinite() {
//             println!("was infinite or nan: {}", shape);
//             shape = 0.1;
//         }
//         if rate.is_nan() || rate == 0.0 || rate.is_infinite(){
//             println!("was infinite or nan: {}", rate);
//             rate = 0.1;
//         }
//         // println!("{} is shape and {} is rate", shape, rate);
//         let gamma = Gamma::new(shape.abs() as f64, rate.abs() as f64)
//             .unwrap();
//
//         let base = self.data
//             .mapv(|g| gamma.ln_pdf(g as f64) as f32);
//
//         let option: f32= base.mean().unwrap();
//
//         Ok(-option)
//     }
// }
//
//
// impl Gradient for GammaOptimizer {
//     type Param = ArrayBase<OwnedRepr<f32>, Dim<[usize; 1]>>;
//     type Gradient = ArrayBase<OwnedRepr<f32>, Dim<[usize; 1]>>;
//
//     fn gradient(&self, params: &Self::Param) -> Result<Self::Gradient, Error> {
//         let shape = params[0];
//         let scale = params[1];
//         let n = self.data.len() as f32;
//         let mut d_shape = self.data.mapv(|x| Self::d_shape(shape, scale, x));
//         let d_shape = d_shape.sum();
//         let mut d_scale = self.data.mapv(|x| Self::d_scale(shape, scale, x));
//         let d_scale = d_scale.sum();
//         Ok(Array1::from(vec![d_shape, d_scale]) / n)
//     }
// }
//
// impl GammaOptimizer {
//     fn d_shape(shape: f32, scale: f32, x: f32) -> f32 {
//         (1.0 / (scale.powf(shape) * gamma(shape as f64) as f32)) * x.powf(shape - 1.0) * (x.ln() - digamma(shape as f64) as f32) * (-x / scale).exp() as f32
//     }
//
//     fn d_scale(shape: f32, scale: f32, x: f32) -> f32 {
//         -(shape / (scale.powf(shape + 1.0) * gamma(shape as f64) as f32)) * x.powf(shape) * (-x / scale).exp() as f32
//     }
// }
//
// #[test]
// fn calculate_dm_dd() {
//     // I will calculate the distribution for how each moment is changing over time, and then
//     // for the next value, sample each distribution. This means having a sequence for the moment changing,
//     // estimating the distribution for this sequence, and then sampling it to forecast the change
//     // in the distribution. Then I have some m predictions about the m moments that I can use. Probably,
//     // instead of just sampling one value, I will sample a few values and take the average.
//     //
//     // So for the original sequence, as well as some n derivatives, this process can be carried out.
//     // So then for each of the original sequence and the first n derivatives, we have a prediction calculated
//     // from the first m moments of those sequences, and we can take the sequence and add it to the following:
//     // sum up all of the derivatives and add it to the forecast from the moment.
//
//     // 1st deriv 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 2.0
//     // 2nd deriv 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 1.0
//     // 3rd deriv 0.0, 0.0, 0.0, 0.0, 1.0, -2.0, 2.0
//
//     let base1 = Array::from_vec(vec![
//         24.0, 23.0, 22.0, 21.0,
//         20.0, 19.0, 18.0, 17.0,
//
//         18.0, 19.0, 20.0, 21.0,
//         23.0, 24.0, 25.0,
//
//         23.0, 24.0, 25.0,
//         26.0, 27.0, 28.0, 29.0,
//         30.0, 31.0, 32.0, 33.0,
//         34.0, 35.0, 36.0, 37.0,
//         38.0, 39.0, 40.0, 41.0
//     ]);
//
//     // min and max need to be set according to the variance
//     let ndarray = normalize_ndarray(base1.view(), 1.0, 100.0);
//     let gamma = GammaOptimizer {
//         data: ndarray.0,
//     };
//
//     let shape = &gamma.data.mean().unwrap().powf(2.0) / gamma.data.var(1.0);
//     let scale = &gamma.data.var(1.0) / gamma.data.mean().unwrap();
//
//     let search = MoreThuenteLineSearch::new();
//     let mut descent = SteepestDescent::new(
//         search
//     );
//     let out = Executor::new(
//         gamma,
//         descent,
//     );
//
//
//     let optim = out.configure(|state| {
//         state.param(array![shape, scale])
//             .max_iters(15000.0 as u64)
//     }).run().unwrap();
//
//     let base = optim.state.param.unwrap();
//
//     let mut rand = rand::thread_rng();
//     println!("{} is the shape and rate", &base);
//     let out = Gamma::new(base[0].abs() as f64, base[1].abs() as f64).unwrap();
//     let scaler = ndarray.1;
//     let mut v = vec![];
//     for i in 0..100 {
//         let x = out.sample(&mut rand);
//         v.push(x as f32);
//     }
//
//     let arr = Array::from_vec(v);
//     let out = undo_normalization(17.0, 41.0, arr);
//     println!("{:?} is out", out);
//
// }
//
// fn undo_normalization(min: f32, max: f32, values: Array1<f32>) -> Array1<f32> {
//     values.map(|v| {
//         (v * (max - min)) + min
//     })
// }
//
// fn normalize_ndarray(array: ArrayView<f32, Ix1>, min: f32, max: f32) -> (Array<f32, Ix1>, LinearScaler<f32>) {
//     println!("{} is min and {} is max", min, max);
//     let params = LinearScaler::min_max_range(min, max);
//     let data = DatasetBase::from(array.insert_axis(Axis(1)));
//     println!("{:?} is data", data);
//     let fit = params.fit(&data).unwrap();
//     let base = fit.transform(data);
//     println!("{:?} is formatted", base);
//     let mut transformed = base.records();
//     let out = transformed.clone().remove_axis(Axis(1));
//     println!("{:?} is normalized", out);
//     (out, fit)
// }
//
