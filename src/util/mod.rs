use std::future::Future;
use std::hash::Hash;
use bevy::log::error;
use tokio::runtime::{Handle, Runtime};
use std::collections::{HashMap, HashSet};
use bevy::prelude::Color;

pub fn group_by_key<K, V>(map: Vec<(K, V)>) -> HashMap<K, HashSet<V>>
    where
        K: Eq + Hash,
        V: Clone + Hash + Eq
{
    let mut result: HashMap<K, HashSet<V>> = HashMap::new();
    for (key, value) in map.into_iter() {
        if result.contains_key(&key) {
            result.get_mut(&key)
                .map(|f| {
                    f.insert(value);
                });
        } else {
            let mut v = HashSet::new();
            v.insert(value);
            result.insert(key, v);
        }
    }
    result
}

pub fn gen_color_from_list(length: f32) -> Vec<Color> {
    let range = 0..length as usize;
    range.map(|i| {
            let hue = i as f32 / length as f32;
            let saturation = 0.5;
            let lightness = 0.5;
            Color::Hsla {
                hue,
                saturation,
                lightness,
                alpha: 175.0,
            }
        })
        .collect()

}

pub fn run_blocking<F: Future>(fn_to_run: F) -> F::Output {
    get_create_runtime(fn_to_run)
}

pub fn get_create_runtime<F: Future>(fn_to_run: F) -> F::Output {
    let handle = Handle::try_current();
    return if handle.is_ok() {
        handle.unwrap().block_on(fn_to_run)
    } else {
        error!("Tokio runtime not present: {:?}", handle.err().unwrap());
        let out = Runtime::new().map(|runtime| {
            runtime.block_on(fn_to_run)
        });
        if out.is_err() {
            panic!("Could not run future!");
        }
        out.unwrap()
    }
}

pub(crate) fn add_or_insert<T, U>(
    key_value: &T,
    group_value: U,
    mut values: &mut HashMap<T, HashSet<U>>
)
where
    T: Hash + Eq + Clone,
    U: Eq + Hash
{
    if values.get(key_value).is_none() {
        values.insert(key_value.clone(), HashSet::from([group_value]));
    } else {
        if values.get(key_value).filter(|c| c.contains(&group_value)).is_none() {
            values.get_mut(key_value)
                .map(|indices| indices.insert(group_value));
        }
    }
}
