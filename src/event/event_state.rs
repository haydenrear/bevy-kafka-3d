use bevy::prelude::{Color, Commands, Component, Entity, Resource};
use std::marker::PhantomData;
use std::fmt::Debug;
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::menu::ui_menu_event::change_style::ChangeStyleTypes;
use crate::event::event_propagation::ChangePropagation;

/// From the event descriptor, create behaviors that will change the state.
pub trait StateChangeFactory<T, A, C, UpdateComponent, U: UpdateStateInPlace<UpdateComponent> = ()>: Sized + Resource
    where
        T: EventData,
        A: EventArgs,
        C: Component,
        UpdateComponent: Component
{
    fn current_state(current: &EventDescriptor<T, A, C>) -> Vec<NextStateChange<U, UpdateComponent>>;
}

/// If the UpdateStateInPlace contains a struct that converts from certain components to other
/// components
pub trait UpdateStateInPlace<T> {
    fn update_state(&self, commands: &mut Commands, value: &mut T);
}

/// Modularizes the state change.
pub struct NextStateChange<T: UpdateStateInPlace<U>, U: Component + Send + Sync + 'static> {
    pub(crate) entity: Entity,
    pub(crate) next_state: T,
    pub(crate) phantom: PhantomData<U>
}

/// The action of updating the component. The next state can delegate further, for instance if
/// the state being updated is not a component.
impl <T: UpdateStateInPlace<U>, U: Component + Send + Sync + 'static> NextStateChange<T, U> {
    pub(crate) fn update_state(&self, commands: &mut Commands, value: &mut U) {
        self.next_state.update_state(commands, value);
    }
}

#[derive(Clone, Debug)]
pub enum HoverStateChange {
    ColoredHover {
        color: Color
    },
    None,
}

#[derive(Clone, Debug)]
pub enum StateChange {
    ChangeComponentColor(Color, ChangePropagation),
    ChangeComponentStyle(ChangeStyleTypes, ChangePropagation),
    None,
}

#[derive(Clone, Debug)]
pub struct Update<T>
    where T: Clone + Debug + Send + Sync + Default
{
    pub(crate) update_to: Option<T>,
}

impl <T> UpdateStateInPlace<T> for Update<T>
    where T: Clone + Debug + Send + Sync + Default
{
    fn update_state(&self, commands: &mut Commands, value: &mut T) {
        *value = self.update_to.as_ref().unwrap().clone();
    }
}
