use std::fmt::Error;
use bevy::prelude::info;
use bevy::prelude::KeyCode::P;
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use ndarray::{Array, ArrayBase, ArrayD, IxDyn, OwnedRepr};
use serde_json::Result as JsonResult;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct MetricMessage {
    pub(crate) shape: Vec<usize>,
    pub(crate) data: Vec<f64>,
    pub(crate) metric_name: String,
    pub(crate) included: Vec<u32>
}

pub(crate) fn get_arr(in_value: &str) -> Option<ArrayBase<OwnedRepr<f64>, IxDyn>> {
    get_metric_message(in_value)
        .map(|tensor| {
            let shape = tensor.shape;
            let data = tensor.data;
            ArrayD::from_shape_vec(shape.as_slice(), data)
                .or_else(|err| {
                    println!("Error deserializing array: {:?}", err);
                    info!("Error deserializing array: {:?}", err);
                    Err(err)
                })
                .ok()
        })
        .flatten()
}

pub(crate) fn get_metric_message(in_value: &str) -> Option<MetricMessage> {
    serde_json::from_str::<MetricMessage>(in_value)
        .or_else(|err| {
            println!("Error deserializing array: {:?}", err);
            info!("Error deserializing array: {:?}", err);
            Err(err)
        })
        .ok()
}