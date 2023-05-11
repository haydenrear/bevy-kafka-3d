use std::fmt::Error;
use std::future::Future;
use bevy::utils::{HashMap, HashSet};
use std::hash::Hash;
use bevy::log::error;
use tokio::runtime::{Handle, Runtime};

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