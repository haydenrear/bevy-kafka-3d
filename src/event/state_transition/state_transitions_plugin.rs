use bevy::prelude::{App, OnEnter, Plugin};
use bevy::prelude::*;
use std::fmt::Debug;
use crate::event::state_transition::get_state_transitions::GetStateTransitions;
use crate::event::state_transition::state_transition_types::{InsertBaseMenuDisplayTransitions, InsertCollapsableDisplayTransitions, InsertDropdownDisplayTransitions, InsertGraphMenuStateTransitions, InsertSelectStateTransitions, InsertVisibleGraphStateTransitions, InsertVisibleNetworkStateTransitions};
use crate::event::state_transition::state_transitions_system::InsertStateTransitions;
use crate::ui_components::ui_menu_component::{create_menu, populate_options_builder};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TransitionsState {
    #[default]
    AddResources,
    PopulateOptionsBuilder,
    InsertStateTransitions,
    CheckDynamicStateTransitions,
    DummyState
}

pub struct InsertStateTransitionsPlugin;

impl Plugin for InsertStateTransitionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<TransitionsState>()
            .add_startup_system(create_menu)
            .add_systems(
                OnEnter(TransitionsState::PopulateOptionsBuilder),
                populate_options_builder
            )
            .add_systems(
                OnEnter(TransitionsState::InsertStateTransitions),
                InsertCollapsableDisplayTransitions::insert_state_transitions
            )
            .add_systems(
                OnEnter(TransitionsState::InsertStateTransitions),
                InsertSelectStateTransitions::insert_state_transitions
            )
            .add_systems(
                OnEnter(TransitionsState::InsertStateTransitions),
                InsertDropdownDisplayTransitions::insert_state_transitions
            )
            .add_systems(
                OnEnter(TransitionsState::InsertStateTransitions),
                InsertBaseMenuDisplayTransitions::insert_state_transitions
            )
            .add_systems(
                OnEnter(TransitionsState::InsertStateTransitions),
                InsertVisibleGraphStateTransitions::insert_state_transitions
            )
            .add_systems(
                OnEnter(TransitionsState::InsertStateTransitions),
                InsertVisibleNetworkStateTransitions::insert_state_transitions
            )
            // dynamic state transitions
            .add_systems(
                OnEnter(TransitionsState::CheckDynamicStateTransitions),
                InsertGraphMenuStateTransitions::insert_state_transitions
            )
        ;
    }
}

