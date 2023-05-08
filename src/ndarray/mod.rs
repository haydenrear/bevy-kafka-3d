use std::fmt::Error;
use bevy::prelude::{error, info};
use bevy::prelude::KeyCode::P;
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use ndarray::{Array, ArrayBase, ArrayD, IxDyn, OwnedRepr};
use serde_json::Result as JsonResult;
use crate::data_subscriber::metric_event::MetricMessage;

pub(crate) fn get_arr(in_value: &str) -> Option<ArrayBase<OwnedRepr<f32>, IxDyn>> {
    get_metric_message(in_value)
        .map(|tensor| {
            let shape = tensor.shape;
            let data = tensor.data;
            data.lock()
                .or_else(|e| {
                    error!("Mutex error: {:?}", e);
                    Err(e)
                })
                .ok()
                .map(|data| {
                    data.as_ref().map(|data| {
                        ArrayD::from_shape_vec(shape.as_slice(), data.to_owned())
                            .or_else(|err| {
                                println!("Error deserializing array: {:?}", err);
                                info!("Error deserializing array: {:?}", err);
                                Err(err)
                            })
                            .ok()
                    })
                })
                .flatten()
                .flatten()
        })
        .flatten()
}

pub(crate) fn get_arr_from_vec(data: Vec<f32>, size: &Vec<usize>) -> Option<ArrayD<f32>> {
    ArrayD::from_shape_vec(size.as_slice(), data)
        .ok()
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