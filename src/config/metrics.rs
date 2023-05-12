use serde::Deserialize;
use std::collections::HashMap;
use crate::graph::{GraphDimType, GridAxis};
use crate::metrics::network_metrics::MetricTypeMatcher;

#[derive(Deserialize, Default)]
pub struct MetricsConfiguration {
    pub(crate) metric_type: HashMap<MetricTypeMatcher, String>,
    pub(crate) dim_type: HashMap<GraphDimType, Vec<String>>,
    pub(crate) dim_axis: HashMap<GridAxis, Vec<String>>,
}

impl MetricsConfiguration {

    pub(crate) fn get_dim_type(&self, column: &str) -> GraphDimType {
        Self::get_value(&self.dim_type, column)
    }

    pub(crate) fn get_grid_axis(&self, column: &str) -> GridAxis {
        Self::get_value(&self.dim_axis, column)
    }

    pub(crate) fn get_value<T: Default + Clone>(map: &HashMap<T, Vec<String>>, to_match: &str) -> T {
        map.iter()
            .filter(|(dim_type, names)| names.iter()
                .any(|name| matches!(to_match, name))
            )
            .map(|(dim_type, _)| dim_type.clone())
            .next()
            .or(Some(T::default()))
            .unwrap()
    }

}
