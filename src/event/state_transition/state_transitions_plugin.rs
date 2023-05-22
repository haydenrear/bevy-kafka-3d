use bevy::prelude::{App, IntoSystemAppConfig, OnEnter, Plugin};
use bevy::prelude::*;
use std::fmt::Debug;
use crate::event::state_transition::get_state_transitions::GetStateTransitions;
use crate::event::state_transition::state_transitions_system::{InsertBaseMenuDisplayTransitions, InsertCollapsableDisplayTransitions, InsertDropdownDisplayTransitions, InsertSelectStateTransitions, InsertStateTransitions, InsertVisibleGraphStateTransitions, InsertVisibleNetworkStateTransitions};
use crate::ui_components::ui_menu_component::{create_menu, populate_options_builder};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TransitionPopulationStartupState {
    #[default]
    AddResources,
    PopulateOptionsBuilder,
    InsertStateTransitions,
}

pub struct InsertStateTransitionsPlugin;

impl Plugin for InsertStateTransitionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<TransitionPopulationStartupState>()
            .add_startup_system(create_menu)
            .add_system(populate_options_builder
                .in_schedule(OnEnter(TransitionPopulationStartupState::PopulateOptionsBuilder))
            )
            .add_system(InsertCollapsableDisplayTransitions::insert_state_transitions
                .in_schedule(OnEnter(TransitionPopulationStartupState::InsertStateTransitions))
            )
            .add_system(InsertSelectStateTransitions::insert_state_transitions
                .in_schedule(OnEnter(TransitionPopulationStartupState::InsertStateTransitions))
            )
            .add_system(InsertDropdownDisplayTransitions::insert_state_transitions
                .in_schedule(OnEnter(TransitionPopulationStartupState::InsertStateTransitions))
            )
            .add_system(InsertBaseMenuDisplayTransitions::insert_state_transitions
                .in_schedule(OnEnter(TransitionPopulationStartupState::InsertStateTransitions))
            )
            .add_system(InsertVisibleGraphStateTransitions::insert_state_transitions
                .in_schedule(OnEnter(TransitionPopulationStartupState::InsertStateTransitions))
            )
            .add_system(InsertVisibleNetworkStateTransitions::insert_state_transitions
                .in_schedule(OnEnter(TransitionPopulationStartupState::InsertStateTransitions))
            );
    }
}
