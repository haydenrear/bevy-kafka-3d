use std::collections::HashMap;
use bevy::ecs::system::CommandQueue;
use bevy::prelude::{Commands, World};
use ndarray::{arr1, arr2, arr3, array, Array2, Array3, Ix2, s};
use crate::data_subscriber::metric_event::MetricsState;
use crate::metrics::network_metrics::HistoricalData;
use crate::ndarray::{get_arr, get_arr_from_vec, get_metric_message};
use crate::test::config_test::get_metric_message_test;
use crate::test::TestComponent;

const TEST_VALUE: &'static str = "{ \"metric_name\": \"name\",\"included\": [1,2,3], \"shape\": [2, 20, 20], \"data\": [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]}";

const TEST_VALUE_TWO: &'static str = "{\"metric_name\": \"this_metric\", \"included\": [1,2,3,4], \"shape\": [1, 2, 3, 4], \"data\": [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]}";

#[test]
fn get_metric_message_test_test() {
    let option = get_metric_message_test(TEST_VALUE);
    assert!(option.is_some());
    let f = option.unwrap().get_data();
    assert_ne!(f.len(), 0);
}

#[test]
fn test_mutable() {
    let mut metrics = MetricsState::default();
    let mut queue = CommandQueue::default();
    let world = World::default();
    let mut commands =
         Commands::new(&mut queue, &world);
    let e = commands.spawn(TestComponent::default()).id();
    metrics.entities.insert("test".to_string(), (e, 1));
    metrics.increment_entity("test");
    assert_eq!(metrics.entities.get("test").unwrap().1, 2);
}

#[test]
fn test_serialize() {
    let arr = get_arr(TEST_VALUE);
    assert!(arr.is_some());
    assert_eq!(arr.as_ref().unwrap().shape().clone(), [2,20,20]);
    let metric = get_metric_message(TEST_VALUE);
    assert!(metric.is_some());
    let metric = metric.unwrap();
    assert_eq!(metric.metric_name, "name".to_string());
    assert_eq!(metric.included, vec![1,2,3]);
}

#[test]
fn test_serialize_two() {
    let arr = get_arr(TEST_VALUE_TWO);
    assert!(arr.is_some());
    assert_eq!(arr.as_ref().unwrap().shape().clone(), [1, 2, 3, 4]);
    let metric = get_metric_message(TEST_VALUE_TWO);
    assert!(metric.is_some());
    let metric = metric.unwrap();
    assert_eq!(metric.metric_name, "this_metric".to_string());
    assert_eq!(metric.included, vec![1, 2, 3, 4])
}

#[test]
fn test_get_arr() {
    let option = get_arr_from_vec(vec![0.0, 1.0], &vec![2]);
    assert!(option.is_ok());
}

#[test]
fn test_historical_data2x2() {
    let mut historical_data = HistoricalData::new(vec![2, 2], HashMap::from([("1".to_string(), 0), ("2".to_string(), 1)]));
    let array = arr3(&[[[0.0, 0.0], [0.0, 0.0]]]);
    let mut base: &[f32] = array.as_slice().unwrap();
    assert_eq!(historical_data.data.as_slice().unwrap(), base);
    historical_data.extend(arr2(&[[1.0, 2.0], [3.0, 4.0]]).into_dyn(), 1);
    let array = arr3(&[[[0.0, 0.0], [0.0, 0.0]], [[1.0, 2.0], [3.0, 4.0]]]);
    base = array.as_slice().unwrap();
    assert_eq!(historical_data.data.as_slice().unwrap(), base);
    historical_data.extend(arr2(&[[5.0, 6.0], [7.0, 8.0]]).into_dyn(), 2);
    let array = arr3(&[[[0.0, 0.0], [0.0, 0.0]], [[1.0, 2.0], [3.0, 4.0]], [[5.0, 6.0], [7.0, 8.0]]]);
    let array = array.as_slice();
    base = array.unwrap();
    assert_eq!(historical_data.data.as_slice().unwrap(), base);

    let (first, second) = historical_data.retrieve_values("1", 2).unwrap();
    println!("{:?} is first and {:?} is second", &first, &second);
    assert_eq!(first, arr1(&[1.0, 2.0]));
    assert_eq!(second, arr1(&[5.0, 6.0]));

    let out = historical_data.retrieve_historical("1");
    let out_assert = arr2(&[[0.0, 0.0], [1.0, 2.0], [5.0, 6.0]]);
    assert_eq!(out.unwrap().as_slice().unwrap(), out_assert.as_slice().unwrap());

}

#[test]
fn test_historical_data1d() {
    let mut historical_data = HistoricalData::new(vec![2], HashMap::from([("1".to_string(), 0), ("2".to_string(), 1)]));
    let array = arr1(&[0.0, 0.0]);
    let mut base: &[f32] = array.as_slice().unwrap();
    assert_eq!(historical_data.data.as_slice().unwrap(), base);
    historical_data.extend(arr1(&[1.0, 2.0]).into_dyn(), 1);
    let array = arr2(&[[0.0, 0.0], [1.0, 2.0]]);
    base = array.as_slice().unwrap();
    assert_eq!(historical_data.data.as_slice().unwrap(), base);
    historical_data.extend(arr1(&[5.0, 6.0]).into_dyn(), 2);
    let array = arr2(&[[0.0, 0.0], [1.0, 2.0], [5.0, 6.0]]);
    let array = array.as_slice();
    base = array.unwrap();
    assert_eq!(historical_data.data.as_slice().unwrap(), base);

    let (first, second) = historical_data.retrieve_values("1", 2).unwrap();
    println!("{:?} is first and {:?} is second", &first, &second);
    assert_eq!(first, arr1(&[1.0]));
    assert_eq!(second, arr1(&[5.0]));

    let out = historical_data.retrieve_historical("1");
    let out_assert = arr1(&[0.0, 1.0, 5.0]);
    assert_eq!(out.unwrap().as_slice().unwrap(), out_assert.as_slice().unwrap());
    let out = historical_data.retrieve_historical_1d("1");
    assert_eq!(out.len(), 1);
    let out_assert = arr1(&[0.0, 1.0, 5.0]);
    assert_eq!(out.get(0).unwrap().as_slice().unwrap(), out_assert.as_slice().unwrap());

}
