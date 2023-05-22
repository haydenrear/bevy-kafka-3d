use std::marker::PhantomData;
use bevy::prelude::Component;
use crate::event::event_descriptor::{EventArgs, EventData};
use crate::event::event_state::Context;
use crate::menu::ui_menu_event::next_action::Matches;
use crate::menu::ui_menu_event::transition_groups::TransitionGroup;
use crate::menu::ui_menu_event::state_change_factory::EntitiesStateTypes;

/// Any arbitrary state transition for any arbitrary component and context.
#[derive(Debug)]
pub struct EntityComponentStateTransition<
    StateMachineT,
    Ctx,
    EventArgsT,
    TransitionGroupComponentT,
    StateComponentT,
    FilterMatchesT,
    UpdateComponentT = StateComponentT,
    UpdateMatchesT = FilterMatchesT,
>
    where
        Ctx: Context,
        EventArgsT: EventArgs,
        UpdateMatchesT: Matches<UpdateComponentT>,
        FilterMatchesT: Matches<StateComponentT>,
        StateMachineT: EventData,
        TransitionGroupComponentT: TransitionGroup
{
    pub(crate) entity_to_change: EntitiesStateTypes<UpdateComponentT, StateMachineT, Ctx, EventArgsT>,
    // filter for whether or not to send events at all.
    pub(crate) filter_state: FilterMatchesT,
    // filter for that entity to be changed, whether it is the parent, child, etc..
    pub(crate) current_state_filter: UpdateMatchesT,
    pub(crate) filter_component: PhantomData<TransitionGroupComponentT>,
    pub(crate) state_component: PhantomData<StateComponentT>,
}

/// Paricular types of state transitions, that are associated with a particular state machine, component, and transition group.
#[derive(Component, Debug)]
pub struct UiEntityComponentStateTransitions<
    StateMachineT,
    Ctx,
    EventArgsT,
    TransitionGroupComponentT,
    StateComponentT,
    FilterMatchesT,
    UpdateComponentT = StateComponentT,
    UpdateMatchesT = FilterMatchesT,
>
where
    Ctx: Context,
    EventArgsT: EventArgs,
    FilterMatchesT: Matches<StateComponentT>,
    UpdateMatchesT: Matches<UpdateComponentT>,
    StateMachineT: EventData,
    TransitionGroupComponentT: TransitionGroup
{
    pub(crate) transitions: Vec<EntityComponentStateTransition<
        StateMachineT,
        Ctx,
        EventArgsT,
        TransitionGroupComponentT,
        StateComponentT,
        FilterMatchesT,
        UpdateComponentT,
        UpdateMatchesT,
    >>,
    pub(crate) state_component: PhantomData<StateComponentT>,
}
