use bevy::prelude::{Commands, Component, ResMut};
use std::fmt::Debug;
use bevy::log::info;
use crate::event::event_state::{Context, UpdateStateInPlace};
use crate::menu::MetricsConfigurationOption;

impl <T, Ctx> UpdateStateInPlace<MetricsConfigurationOption<T>, Ctx>
for MetricsConfigurationOption<T>
where T: Component + Send + Sync + Clone + Debug + Default + 'static,
    Ctx: Context
{
    fn update_state(&self,commands: &mut Commands, value: &mut MetricsConfigurationOption<T>, ctx: &mut ResMut<Ctx>) {
        info!("Updating state from {:?} to {:?}.", value, &self);
        *value = self.clone()
    }
}
