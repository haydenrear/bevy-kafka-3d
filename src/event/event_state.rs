use bevy::prelude::{Color, Commands, Component, Entity, info, ResMut, Resource, Style, Visibility};
use std::marker::PhantomData;
use std::fmt::Debug;
use bevy::ecs::query::{ReadOnlyWorldQuery, WorldQuery};
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::menu::{Menu, MetricsConfigurationOption};
use crate::menu::ui_menu_event::change_style::UiChangeTypes;
use crate::menu::ui_menu_event::next_action::UiComponentState;
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiEventArgs;
use crate::menu::ui_menu_event::ui_state_change::{ChangeVisible, GlobalState, StateAdviser};
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
    fn current_state(
        current: &EventDescriptor<T, A, C>,
        ctx: &mut ResMut<Ctx>
    ) -> Vec<NextStateChange<U, UpdateComponent, Ctx>>;
}

/// From the event descriptor, create behaviors that will change the state. This can be hooked into downstream
/// to perform some further actions, for example by adding a menu (using Changed<>)
pub trait InsertComponentChangeFactory<EventDataT, EventArgsT, InsertComponentComponent, StateAdviserComponentT, Ctx, InsertComponentT>: Sized + Resource
    where
        EventDataT: EventData,
        EventArgsT: EventArgs,
        StateAdviserComponentT: StateAdviser<InsertComponentComponent>,
        InsertComponentComponent: Component + Debug + Clone,
        Ctx: Context + Debug,
        InsertComponentT: InsertComponent<InsertComponentComponent, StateAdviserComponentT, Ctx>
{
    fn current_state(
        current: &EventDescriptor<EventDataT, EventArgsT, InsertComponentComponent>,
        ctx: &mut ResMut<Ctx>
    ) -> Vec<InsertComponentT>;
}

/// If the UpdateStateInPlace contains a struct that converts from certain components to other
/// components
pub trait UpdateStateInPlace<T, Ctx: Context>: Debug {
    fn update_state(&self, commands: &mut Commands, value: &mut T, ctx: &mut ResMut<Ctx>);
}

pub trait InsertComponent<ToInsertComponentT, StateAdviserT, Ctx>: Debug
where
    ToInsertComponentT: Clone + Component,
    StateAdviserT: StateAdviser<ToInsertComponentT>,
    Ctx: Context
{
    fn insert_update_components(
        &self,
        mut commands: &mut Commands,
        value: &ToInsertComponentT,
        ctx: &mut ResMut<Ctx>,
        entity: Entity,
        current_states: &StateAdviserT
    ) {
        info!("Inserting component into {:?}.", entity);
        let value = current_states.advise(&mut commands, value);
        let _ = commands.get_entity(entity)
            .as_mut()
            .map(|entity_cmd| {
                entity_cmd.insert(value);
            });
    }

    fn adviser_component(&self) -> Entity;

    fn entity_component(&self) -> Entity;

    fn next_state(&self) -> &ToInsertComponentT;

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

/// Modularize the state change.
#[derive(Debug)]
pub struct NextComponentInsert<InsertComponentT, AdviserComponentT, Ctx>
    where
        InsertComponentT: Component + Send + Sync + 'static + Clone + Debug,
        AdviserComponentT: Component + Send + Sync + 'static + Clone + Debug,
        Ctx: Context + Debug
{
    pub(crate) insert_component_entity: Entity,
    pub(crate) adviser_component_entity: Entity,
    pub(crate) next_state: InsertComponentT,
    pub(crate) phantom: PhantomData<AdviserComponentT>,
    pub(crate) phantom_ctx: PhantomData<Ctx>
}

impl <NextEventComponentT, AdviserComponentT, Ctx> InsertComponent<NextEventComponentT, AdviserComponentT, Ctx>
for NextComponentInsert<NextEventComponentT, AdviserComponentT, Ctx>
    where
        NextEventComponentT: Component + Debug + Clone,
        Ctx: Context + Debug + Clone,
        AdviserComponentT: StateAdviser<NextEventComponentT> + Clone
{

    fn adviser_component(&self) -> Entity {
        self.adviser_component_entity
    }

    fn entity_component(&self) -> Entity {
        self.insert_component_entity
    }

    fn next_state(&self) -> &NextEventComponentT {
        &self.next_state
    }
}

/// The action of updating the component. The next state can delegate further, for instance if
/// the state being updated is not a component.
impl <T: UpdateStateInPlace<U, Ctx>, U, Ctx> NextStateChange<T, U, Ctx>
where
    U: Component + Send + Sync + 'static,
    Ctx: Context
{
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
    fn clicked(&mut self, entity: Entity);
    fn un_clicked(&mut self);
    fn cursor(&mut self, global_stat: &mut ResMut<GlobalState>);
}

#[derive(Clone, Debug)]
pub enum StyleStateChangeEventData {
    ChangeComponentColor(Color),
    ChangeComponentStyle(UiChangeTypes),
    ChangeTextValue,
    None,
}

impl EventData for StyleStateChangeEventData {}

#[derive(Clone, Debug)]
pub enum ComponentChangeEventData {
    ChangeVisible{ to_change: Entity, adviser_component: Entity},
    ChangeGraphingMenu
}


pub trait NextComponentInsertFactory<StateAdviserComponentT: StateAdviser<ComponentChangeT> + Clone, ComponentChangeT: Component + Debug + Clone> {
    fn next(&self, event_args: UiEventArgs) -> NextComponentInsert<ComponentChangeT, StateAdviserComponentT, UiContext>;
}

impl EventData for ComponentChangeEventData {}

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
        info!("Updating to state: {:?} from state: {:?}.", value, self.update_to);
        *value = self.update_to.as_ref().unwrap().clone();
    }
}

