use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::prelude::{Commands, Component, Entity, info, ResMut, Resource};
use bevy::utils::HashMap;
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_state::{Context, NextStateChange, StateChangeFactory, Update, UpdateStateInPlace};
use crate::menu::{MetricsConfigurationOption, ConfigurationOptionEnum, DataType};
use crate::menu::config_menu_event::interaction_config_event_writer::ConfigOptionContext;
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
    pub(crate) config_option: HashMap<Entity, MetricsConfigurationOption<T>>,
}

impl <T> ConfigurationOptionChange<T>
where T: Component + Send + Sync + Clone + Default + Debug + 'static
{

    pub(crate) fn to_config_option_states(&self) -> Vec<NextConfigurationOptionState<T>>  {

        let mut to_replace: HashMap<Entity, MetricsConfigurationOption<T>> = self.config_option.clone();

        to_replace.into_iter()
            .flat_map(|(entity, option)| {
                if matches!(option, MetricsConfigurationOption::Variance(..)) {
                    return vec![NextConfigurationOptionState::UpdateVariance(option)];
                } else if matches!(option, MetricsConfigurationOption::Concavity(..)) {
                    return vec![NextConfigurationOptionState::UpdateVariance(option)];
                } else if matches!(option, MetricsConfigurationOption::GraphMenu(..)) {
                    return vec![NextConfigurationOptionState::UpdateMenu(option)];
                } else if matches!(option, MetricsConfigurationOption::Metrics(..)) {
                    return vec![NextConfigurationOptionState::UpdateMetrics(option)];
                } else if matches!(option, MetricsConfigurationOption::NetworkMenu(..)) {
                 return vec![NextConfigurationOptionState::UpdateMenu(option)];
                }
                vec![]
            })
            .collect()
    }

}

impl EventData for DataType {}

impl <T: Component + Send + Sync + Default + Clone + Debug + 'static> EventArgs for ConfigurationOptionEventArgs<T> {}

#[derive(Debug)]
pub enum NextConfigurationOptionState<T: Component + Send + Sync + 'static + Clone + Debug + Default> {
    UpdateVariance(MetricsConfigurationOption<T>),
    UpdateConcavity(MetricsConfigurationOption<T>),
    UpdateMetrics(MetricsConfigurationOption<T>),
    UpdateMenu(MetricsConfigurationOption<T>),
    Default
}

impl <T, Ctx> UpdateStateInPlace<MetricsConfigurationOption<T>, Ctx>
for NextConfigurationOptionState<T>
where
    T : Component + Send + Sync + 'static + Clone + Debug + Default,
    Ctx: Context
{
    fn update_state(&self, commands: &mut Commands, value: &mut MetricsConfigurationOption<T>, ctx: &mut ResMut<Ctx>) {
        info!("In update state with {:?}.", value);
        if let NextConfigurationOptionState::UpdateConcavity(node) = self {
            node.update_state(commands, value, ctx);
        } else if let NextConfigurationOptionState::UpdateMenu(node) = self {
            node.update_state(commands, value, ctx);
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct ConfigEventStateFactory;

impl <T: Component + Send + Sync + Clone + Default + Debug + 'static>
StateChangeFactory<
    DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>,
    MetricsConfigurationOption<T>, ConfigOptionContext, NextConfigurationOptionState<T>
>
for ConfigEventStateFactory {
    fn current_state(current: &EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>,
                     context: &mut ResMut<ConfigOptionContext>)
        -> Vec<NextStateChange<NextConfigurationOptionState<T>, MetricsConfigurationOption<T>, ConfigOptionContext>>
    {
        if let ConfigurationOptionEventArgs::Event(
            config,
            entity
        ) = &current.event_args {
            return config.to_config_option_states()
                .into_iter()
                .map(|config| NextStateChange {
                    entity: entity.clone(),
                    next_state: config,
                    phantom: PhantomData::default(),
                    phantom_ctx: Default::default(),
                })
                .collect();
        }
        vec![]
    }
}

