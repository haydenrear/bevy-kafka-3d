use bevy::prelude::{Button, Changed, Entity, Interaction, Style, With};
use crate::event::event_descriptor::EventDescriptor;
use crate::event::event_state::StyleStateChangeEventData;
use crate::menu::ui_menu_event::interaction_ui_event_writer::StateChangeActionTypeStateRetriever;
use crate::menu::ui_menu_event::next_action::UiComponentState;
use crate::menu::ui_menu_event::style_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{StateChangeActionType, UiEntityComponentStateTransitions, UiEventArgs};
use crate::menu::{DraggableComponent, ScrollableComponent, UiComponent};
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

pub type UiComponentStyleFilter = (With<UiComponent>, With<Style>);
pub type UiComponentStyleIxnFilter = (With<UiComponent>, With<Button>, Changed<Interaction>);
pub type DraggableUiComponentFilter = (With<UiComponent>, With<Style>, With<DraggableComponent>);
pub type DraggableUiComponentIxnFilter = (With<UiComponent>, With<Button>, With<DraggableComponent>);
pub type ScrollableUiComponentFilter = (With<UiComponent>, With<Style>, With<ScrollableComponent>);
pub type ScrollableUiComponentIxnFilter = (With<UiComponent>, With<Button>, With<ScrollableComponent>);
pub type ScrollableIxnFilterQuery = (With<UiComponent>, With<Button>, With<ScrollableComponent>);
pub type PropagationQueryFilter<C> = (With<C>);


pub type PropagationQuery<'a, C> = (Entity, &'a C, &'a UiIdentifiableComponent);
pub type StateTransitionsQuery<'a, ComponentT, StateMachineT, MatchesT, Ctx, EventArgsT> =
(Entity,
 &'a UiComponent,
 &'a ComponentT,
 &'a UiIdentifiableComponent,
 &'a UiEntityComponentStateTransitions<StateMachineT, ComponentT, MatchesT, Ctx, EventArgsT>);


pub type StyleUiComponentStateTransitionsQuery<'a> = StateTransitionsQuery<'a, Style, StyleStateChangeEventData, UiComponentState, UiContext, UiEventArgs>;
pub type StylePropagationQuery<'a> = PropagationQuery<'a, Style>;
pub type StylePropagationQueryFilter = PropagationQueryFilter<Style>;


pub type UiStateChange<C, S> = StateChangeActionType<S, C, UiContext, UiEventArgs>;
pub type StyleStateChange = StateChangeActionType<StyleStateChangeEventData, Style, UiContext, UiEventArgs>;
pub type UiStyleComponentStateTransitions = UiEntityComponentStateTransitions<StyleStateChangeEventData, Style, UiComponentState, UiContext, UiEventArgs>;


pub type DraggableStateChangeRetriever = StateChangeActionTypeStateRetriever<
    DraggableUiComponentFilter, DraggableUiComponentIxnFilter, Style,
    UiContext, UiEventArgs, StyleStateChangeEventData, UiComponentState>;

pub type ScrollableStateChangeRetriever = StateChangeActionTypeStateRetriever<
    ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter, Style, UiContext,
    UiEventArgs, StyleStateChangeEventData, UiComponentState>;

pub type ClickEvents = StateChangeActionTypeStateRetriever<
    UiComponentStyleFilter, UiComponentStyleIxnFilter, Style,
    UiContext, UiEventArgs, StyleStateChangeEventData, UiComponentState>;


pub type UiComponentEventDescriptor = EventDescriptor<StyleStateChangeEventData, UiEventArgs, Style>;


pub type UiStyleEntityComponentStateTransitions = UiEntityComponentStateTransitions<
    StyleStateChangeEventData, Style,
    UiComponentState, UiContext,
    UiEventArgs>;

