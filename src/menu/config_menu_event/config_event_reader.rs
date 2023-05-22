use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::prelude::{Component, With};
use crate::event::event_actions::InteractionEventReader;
use crate::menu::config_menu_event::config_event::{ConfigEventStateFactory, ConfigurationOptionEventArgs, NextConfigurationOptionState};
use crate::menu::{DataType, MetricsConfigurationOption};
use crate::menu::config_menu_event::interaction_config_event_writer::NetworkMenuResultBuilder;

pub struct ConfigEventReader<T>
    where T: Component
{
    phantom: PhantomData<T>
}

impl<T> InteractionEventReader<
    DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>,
    MetricsConfigurationOption<T>, ConfigEventStateFactory,
    NextConfigurationOptionState<T>, NetworkMenuResultBuilder,
    (With<MetricsConfigurationOption<T>>)
> for ConfigEventReader<T>
    where T: Component + Clone + Debug + Default
{}