use bevy::prelude::{Color, Commands, Component, Entity, ResMut, Resource};
use std::marker::PhantomData;
use std::fmt::Debug;
use bevy::ecs::query::ReadOnlyWorldQuery;
use crate::cursor_adapter::CursorResource;
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::menu::ui_menu_event::change_style::ChangeStyleTypes;
use crate::event::event_propagation::ChangePropagation;
use crate::menu::ui_menu_event::ui_state_change::GlobalState;
use crate::menu::ui_menu_event::ui_state_change::StateChangeMachine;

/// From the event descriptor, create behaviors that will change the state.
pub trait StateChangeFactory<T, A, C, UpdateComponent, Ctx, U: UpdateStateInPlace<UpdateComponent, Ctx> = ()>: Sized + Resource
    where
        T: EventData,
        A: EventArgs,
        C: Component,
        UpdateComponent: Component,
        Ctx: Context
{
    fn current_state(current: &EventDescriptor<T, A, C>, ctx: &mut ResMut<Ctx>) -> Vec<NextStateChange<U, UpdateComponent, Ctx>>;
}

/// If the UpdateStateInPlace contains a struct that converts from certain components to other
/// components
pub trait UpdateStateInPlace<T, Ctx: Context>: Debug {
    fn update_state(&self, commands: &mut Commands, value: &mut T, ctx: &mut ResMut<Ctx>);
}

/// If the UpdateStateInPlace contains a struct that converts from certain components to other
/// components
pub trait StateUpdate<T>: Debug {
    fn update_state(&self, value: &mut T);
}

#[derive(Debug)]/// Modularizes the state change.
pub struct NextStateChange<T: UpdateStateInPlace<U, Ctx>, U: Component + Send + Sync + 'static, Ctx: Context> {
    pub(crate) entity: Entity,
    pub(crate) next_state: T,
    pub(crate) phantom: PhantomData<U>,
    pub(crate) phantom_ctx: PhantomData<Ctx>
}

/// The action of updating the component. The next state can delegate further, for instance if
/// the state being updated is not a component.
impl <T: UpdateStateInPlace<U, Ctx>, U: Component + Send + Sync + 'static, Ctx: Context> NextStateChange<T, U, Ctx> {
    pub(crate) fn update_state(&self, commands: &mut Commands, value: &mut U, ctx: &mut ResMut<Ctx>) {
        self.next_state.update_state(commands, value, ctx);
    }
}

#[derive(Clone, Debug)]
pub enum HoverStateChange {
    ColoredHover {
        color: Color
    },
    None,
}

pub trait Context: Resource {}

pub trait ClickContext<SelfFilterQuery, InteractionFilterQuery>: Context
where
    SelfFilterQuery: Send + Sync + 'static,
    InteractionFilterQuery: Send + Sync + 'static
{
    fn clicked(&mut self);
    fn un_clicked(&mut self);
    fn cursor(&mut self, global_stat: &mut ResMut<GlobalState>);
}

#[derive(Clone, Debug)]
pub enum StyleStateChangeEventData {
    ChangeComponentColor(Color),
    ChangeComponentStyle(ChangeStyleTypes),
    None,
}

impl EventData for StyleStateChangeEventData {}

#[derive(Clone, Debug)]
pub struct Update<T>
    where T: Clone + Debug + Send + Sync
{
    pub(crate) update_to: Option<T>,
}

impl <T, Ctx> UpdateStateInPlace<T, Ctx> for Update<T>
    where
        T: Clone + Debug + Send + Sync,
        Ctx: Context
{
    fn update_state(&self, commands: &mut Commands, value: &mut T, ctx: &mut ResMut<Ctx>) {
        *value = self.update_to.as_ref().unwrap().clone();
    }
}

