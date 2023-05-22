use std::collections::{HashMap, HashSet};
use crate::util::add_or_insert;

#[test]
fn test_utils() {
    let mut grouped = HashMap::new();
    let mut this_hashset = HashSet::from(["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string()]);
    grouped.insert("key".to_string(), this_hashset.clone());
    add_or_insert(&"key".to_string(), "another".to_string(), &mut grouped);
    this_hashset.insert("another".to_string());
    let compare_hashset = grouped.get("key").unwrap().clone();
    assert_eq!(compare_hashset, this_hashset);
}