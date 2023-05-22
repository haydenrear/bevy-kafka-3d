use bevy::prelude::*;
use bevy::app::{App, Plugin};
use std::fmt::Debug;
use crate::cursor_adapter::{event_merge_propagate, propagate_drag_events, propagate_scroll_events};
use crate::event::event_actions::{EventsSystem, InsertComponentInteractionEventReader, InteractionEventReader};
use crate::event::event_descriptor::EventArgs;
use crate::event::event_state::Context;
use crate::event::state_transition::get_state_transitions::GetStateTransitions;
use crate::event::state_transition::state_transitions_plugin::InsertStateTransitionsPlugin;
use crate::interactions::InteractionEvent;
use crate::menu::{Menu, MetricsConfigurationOption};
use crate::menu::ui_menu_event::interaction_ui_event_reader::UiEventReader;
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::type_alias::event_reader_writer::{DraggableUiComponentIxnFilter, ScrollableIxnFilterQuery, UiComponentEventDescriptor, UiComponentStyleIxnFilter, VisibilityComponentChangeEventReader, VisibilityEventDescriptor};
use crate::menu::ui_menu_event::type_alias::state_change_action_retriever::{ChangeVisibleEventRetriever, ClickEvents, ClickSelectionEventRetriever, DraggableStateChangeRetriever, ScrollableStateChangeRetriever};
use crate::menu::ui_menu_event::ui_state_change;
use crate::menu::ui_menu_event::ui_state_change::{StateChangeMachine, UiClickStateChange};
use crate::ui_components::menu_components::BuildMenuResult;
use crate::menu::ui_menu_event::state_change_factory::StateChangeActionType;
use crate::menu::ui_menu_event::ui_event_writer::interaction_ui_event_writer::{ClickSelectOptions, DragEvents, ScrollEvents, VisibilitySystems};

pub struct UiEventPlugin;

impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InsertStateTransitionsPlugin)
            .insert_resource(BuildMenuResult::default())
            .insert_resource(ClickEvents::default())
            .insert_resource(DraggableStateChangeRetriever::default())
            .insert_resource(ScrollableStateChangeRetriever::default())
            .insert_resource(UiContext::default())
            .insert_resource(ClickSelectOptions::default())
            .insert_resource(ClickSelectionEventRetriever::default())
            .insert_resource(ChangeVisibleEventRetriever::default())
            .add_system(VisibilitySystems::<MetricsConfigurationOption<Menu>>::click_write_events)
            .add_system(VisibilityComponentChangeEventReader::read_events)
            .add_system(ClickEvents::click_write_events)
            .add_system(DragEvents::click_write_events)
            .add_system(ScrollEvents::click_write_events)
            .add_system(ClickSelectOptions::click_write_events)
            .add_system(UiEventReader::read_events)
            .add_system(ui_state_change::hover_event)
            .add_system(event_merge_propagate::<DraggableUiComponentIxnFilter>)
            .add_system(event_merge_propagate::<ScrollableIxnFilterQuery>)
            .add_system(event_merge_propagate::<UiComponentStyleIxnFilter>)
            .add_system(propagate_scroll_events)
            .add_system(propagate_drag_events)
            .add_event::<InteractionEvent<DraggableUiComponentIxnFilter>>()
            .add_event::<InteractionEvent<ScrollableIxnFilterQuery>>()
            .add_event::<InteractionEvent<UiComponentStyleIxnFilter>>()
            .add_event::<VisibilityEventDescriptor>()
            .add_event::<UiComponentEventDescriptor>()
        ;
    }
}

#[derive(Debug)]
pub enum UiEventArgs {
    Event(UiClickStateChange)
}

impl EventArgs for UiEventArgs {}

macro_rules! get_value {
    ($($name:ident),*)  => {
        impl <T, C, Ctx, Args> StateChangeActionType<T, C, Ctx, Args>
        where
            Ctx: Context,
            Args: EventArgs,
            T: StateChangeMachine<C, Ctx, Args> + Send + Sync
        {

            pub(crate) fn get_state_machine<'a>(&'a self) -> &'a T {
                match &self {
                    $(
                        StateChangeActionType::$name{ value, .. } => {
                            &value
                        }
                    )*
                }
            }
        }
    }
}

get_value!(
    Hover, Clicked, Dragged, Scrolled
);
