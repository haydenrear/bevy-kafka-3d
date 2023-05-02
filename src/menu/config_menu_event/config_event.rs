use std::fmt::Debug;
use bevy::prelude::{Component, Entity, Resource};
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_state::{NextStateChange, StateChangeFactory, Update, UpdateStateInPlace};
use crate::menu::{ConfigurationOption, DataType};
use crate::network::{Layer, Node};

#[derive(Debug, Clone)]
pub enum ConfigurationOptionEvent<T: Component + Send + Sync + Clone + Default + Debug + 'static> {
    Event(ConfigurationOption<T>, Entity)
}

impl <T: Component + Send + Sync + Clone + Default + Debug + 'static> ConfigurationOptionEvent<T> {

    pub(crate) fn entity(&self) -> Option<&Entity> {
        if let ConfigurationOptionEvent::Event(_, entity) = &self {
            return Some(entity)
        }
        None
    }

    pub(crate) fn next_configuration_option(&self) -> Option<NextConfigurationOptionState<T>>  {
        if let ConfigurationOptionEvent::Event(concavity , _)  = &self {
            if matches!(concavity, ConfigurationOption::Concavity(_, _)) {
                let concavity: ConfigurationOption<T> = concavity.clone();
                let updated: Update<ConfigurationOption<T>> = Update {
                    update_to: Some(concavity),
                };
                let next_config_state: NextConfigurationOptionState<T> = NextConfigurationOptionState::UpdateConcavity(
                    updated
                );
                return Some(next_config_state);
            }  else if matches!(concavity, ConfigurationOption::Variance(_, _)) {
                let concavity: ConfigurationOption<T> = concavity.clone();
                let updated: Update<ConfigurationOption<T>> = Update {
                    update_to: Some(concavity),
                };
                let next_config_state: NextConfigurationOptionState<T> = NextConfigurationOptionState::UpdateVariance(
                    updated
                );
                return Some(next_config_state);
            }
        }
        None
    }

}

impl EventData for DataType {}

impl <T: Component + Send + Sync + Default + Clone + Debug + 'static> EventArgs for ConfigurationOptionEvent<T> {}

pub enum NextConfigurationOptionState<T: Component + Send + Sync + 'static + Clone + Debug + Default> {
    UpdateVariance(Update<ConfigurationOption<T>>),
    UpdateConcavity(Update<ConfigurationOption<T>>)
}

impl <T: Component + Send + Sync + 'static + Clone + Debug + Default> UpdateStateInPlace<ConfigurationOption<T>> for ConfigurationOption<T> {
    fn update_state(&self, value: &mut ConfigurationOption<T>) {
        *value = self.clone();
    }
}

impl <T: Component + Send + Sync + 'static + Clone + Debug + Default> UpdateStateInPlace<ConfigurationOption<T>> for NextConfigurationOptionState<T> {
    fn update_state(&self, value: &mut ConfigurationOption<T>) {
        if let NextConfigurationOptionState::UpdateConcavity(node) = self {
            node.update_state(value);
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct ConfigEventStateFactory;

impl <T: Component + Send + Sync + Clone + Default + Debug + 'static> StateChangeFactory<DataType, ConfigurationOptionEvent<T>, ConfigurationOption<T>, NextConfigurationOptionState<T>>
for ConfigEventStateFactory {
    fn current_state(current: &EventDescriptor<DataType, ConfigurationOptionEvent<T>, ConfigurationOption<T>>)
        -> Vec<NextStateChange<NextConfigurationOptionState<T>, ConfigurationOption<T>>>
    {
        current.event_args.entity()
            .map(|entity| {
                current.event_args.next_configuration_option()
                    .map(|config| NextStateChange {
                        entity: entity.clone(),
                        next_state: config,
                        phantom: Default::default(),
                    })
            })
            .flatten()
            .into_iter()
            .collect()
    }
}
