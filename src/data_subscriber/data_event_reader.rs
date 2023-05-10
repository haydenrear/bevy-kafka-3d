use std::marker::PhantomData;
use std::os::unix::raw::time_t;
use bevy::prelude::{Commands, Component, Entity, EventReader, Query, Res, ResMut, Resource};
use bevy::utils::HashMap;
use crate::config::ConfigurationProperties;
use crate::data_subscriber::metric_event::NetworkMetricsServiceEvent;
use crate::graph::DataSeries;
use crate::metrics::network_metrics::{Metric, MetricType};
use crate::ndarray::get_arr_from_vec;

#[derive(Resource, Debug, Default)]
pub struct MetricsState {
    pub(crate) entities: HashMap<String, (Entity, u64)>
}

impl MetricsState {

    pub(crate) fn get_entity(&self, name: &str) -> Option<(Entity, u64)> {
        self.entities.get(name)
            .map(|(entity, timestep)| (*entity, *timestep))
    }

    pub(crate) fn increment_entity(&mut self, name: &str) {
        self.entities.get_mut(name)
            .as_mut()
            .map(|entity_state| {
                entity_state.1 += 1;
            });
    }

}

pub fn read_metric_events<T, U>(
    mut commands: Commands,
    mut event_read: EventReader<T>,
    mut metrics_lookup: ResMut<MetricsState>,
    mut config_properties: Res<ConfigurationProperties>,
    mut component_query: Query<(Entity, &mut Metric<U>)>
)
where
    T: NetworkMetricsServiceEvent<U> + Send + Sync + 'static,
    U: Component + Send + Sync + 'static
{

    for mut event in event_read.iter() {
        let metric_name = event.metric_name();
        if metrics_lookup.entities.contains_key(metric_name)  {
            let (entity, timestep) = metrics_lookup.entities
                .get(metric_name)
                .unwrap();
            let _ = component_query.get_mut(entity.clone())
                .as_mut()
                .map(|(entity, metric)| {
                    get_arr_from_vec( event.get_data(), event.get_shape())
                        .map(|arr| metric.historical.extend(arr, timestep + 1))
                });
        } else {
            for (matcher, to_match) in config_properties.metrics.metric_type.iter() {
                if matches!(metric_name, to_match) {

                    let metric_type = matcher.get_metric::<U>();
                    let columns = event.get_columns()
                        .or(Some(HashMap::new()))
                        .unwrap();

                    let mut metric = Metric::<U>::new(
                        event.get_shape().clone(),
                        metric_type,
                        columns
                    );

                    get_arr_from_vec( event.get_data(), event.get_shape())
                        .map(|arr| metric.historical.extend(arr, 0));

                    // TODO: this should wait and the series is created based on changes in menu
                    commands.spawn((metric, DataSeries{
                        drawn: vec![],
                        prev_convergence_times: Default::default(),
                        columns: vec![]
                    }));

                    break;
                }
            }

        }
        metrics_lookup.increment_entity(metric_name);
    }
}