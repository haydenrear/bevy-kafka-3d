use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::prelude::{Commands, Component, Entity, Resource};
use bevy::utils::HashMap;
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_state::{NextStateChange, StateChangeFactory, Update, UpdateStateInPlace};
use crate::menu::{MetricsConfigurationOption, ConfigurationOptionEnum, DataType};
use crate::network::{Layer, Node};

#[derive(Debug, Clone)]
pub enum ConfigurationOptionEventArgs<T>
where T: Component + Send + Sync + Clone + Default + Debug + 'static
{
    Event(ConfigurationOptionChange<T>, Entity)
}

#[derive(Debug, Clone, Default)]
pub struct ConfigurationOptionChange<T>
where T: Component + Send + Sync + Clone + Default + Debug + 'static {
    config_option: Option<HashMap<Entity, MetricsConfigurationOption<T>>>,
}

impl <T> ConfigurationOptionChange<T>
where T: Component + Send + Sync + Clone + Default + Debug + 'static
{

    pub(crate) fn to_config_option_state(&self) -> Vec<NextConfigurationOptionState<T>>  {

        let mut to_replace: Option<HashMap<Entity, MetricsConfigurationOption<T>>> = self.config_option.clone();

        to_replace.into_iter()
            .flat_map(|i| i.into_iter())
            .flat_map(|(entity, option)| {
                if matches!(option, MetricsConfigurationOption::Variance(..)) {
                    return vec![NextConfigurationOptionState::UpdateVariance(option)];
                } else if matches!(option, MetricsConfigurationOption::Concavity(..)) {
                    return vec![NextConfigurationOptionState::UpdateVariance(option)];
                }
                vec![]
            })
            .collect()
    }

}

impl EventData for DataType {}

impl <T: Component + Send + Sync + Default + Clone + Debug + 'static> EventArgs for ConfigurationOptionEventArgs<T> {}

pub enum NextConfigurationOptionState<T: Component + Send + Sync + 'static + Clone + Debug + Default> {
    UpdateVariance(MetricsConfigurationOption<T>),
    UpdateConcavity(MetricsConfigurationOption<T>),
    Default
}

impl <T: Component + Send + Sync + 'static + Clone + Debug + Default> UpdateStateInPlace<MetricsConfigurationOption<T>>
for NextConfigurationOptionState<T> {
    fn update_state(&self, commands: &mut Commands, value: &mut MetricsConfigurationOption<T>) {
        if let NextConfigurationOptionState::UpdateConcavity(node) = self {
            node.update_state(commands, value);
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct ConfigEventStateFactory;

impl <T: Component + Send + Sync + Clone + Default + Debug + 'static>
StateChangeFactory<
    DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>,
    MetricsConfigurationOption<T>, NextConfigurationOptionState<T>
>
for ConfigEventStateFactory {
    fn current_state(current: &EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>)
        -> Vec<NextStateChange<NextConfigurationOptionState<T>, MetricsConfigurationOption<T>>>
    {
        if let ConfigurationOptionEventArgs::Event(
            config,
            entity
        ) = &current.event_args {
            return config.to_config_option_state()
                .into_iter()
                .map(|config| NextStateChange {
                    entity: entity.clone(),
                    next_state: config,
                    phantom: PhantomData::default()
                })
                .collect();
        }
        vec![]
    }
}
