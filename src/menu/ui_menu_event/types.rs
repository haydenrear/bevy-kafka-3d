use bevy::prelude::{Button, Changed, Entity, Interaction, Style, With};
use bevy::text::Text;
use crate::event::event_descriptor::EventDescriptor;
use crate::event::event_state::StyleStateChangeEventData;
use crate::menu::ui_menu_event::interaction_ui_event_writer::{ClickSelectOptions, StateChangeActionTypeStateRetriever};
use crate::menu::ui_menu_event::next_action::UiComponentState;
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{EntityComponentStateTransition, PropagateDisplay, PropagateDraggable, PropagateScrollable, SelectOptions, StateChangeActionType, UiEntityComponentStateTransitions, UiEventArgs};
use crate::menu::{DraggableComponent, ScrollableComponent, UiComponent};
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

pub type UiComponentStyleFilter = (With<UiComponent>, With<Style>);
pub type UiComponentStyleIxnFilter = (With<UiComponent>, With<Button>, Changed<Interaction>);

pub type DraggableUiComponentFilter = (With<UiComponent>, With<Style>, With<DraggableComponent>);
pub type DraggableUiComponentIxnFilter = (With<UiComponent>, With<Button>, With<DraggableComponent>);

pub type ScrollableUiComponentFilter = (With<UiComponent>, With<Style>, With<ScrollableComponent>);
pub type ScrollableIxnFilterQuery = (With<UiComponent>, With<ScrollableComponent>);

pub type PropagationQueryFilter<C> = (With<C>);
pub type PropagationQuery<'a, C> = (Entity, &'a C, &'a UiIdentifiableComponent);
pub type StateTransitionsQuery<'a, ComponentT, StateMachineT, MatchesT, Ctx, EventArgsT, TransitionGroupT> =
(Entity,
 &'a UiComponent,
 &'a ComponentT,
 &'a UiIdentifiableComponent,
 &'a UiEntityComponentStateTransitions<StateMachineT, ComponentT, MatchesT, Ctx, EventArgsT, TransitionGroupT>);


pub type StyleUiComponentStateTransitionsQuery<'a> = StateTransitionsQuery<'a, Style, StyleStateChangeEventData, UiComponentState, UiContext, UiEventArgs, PropagateDisplay>;
pub type StylePropagationQuery<'a> = PropagationQuery<'a, Style>;
pub type StylePropagationQueryFilter = PropagationQueryFilter<Style>;

pub type PropagateStateTransitionsQuery<'a, TransitionGroupT> = StateTransitionsQuery<'a, Style, StyleStateChangeEventData, UiComponentState, UiContext, UiEventArgs, TransitionGroupT>;

pub type UiStateChange<C, S> = StateChangeActionType<S, C, UiContext, UiEventArgs>;
pub type StyleStateChange = StateChangeActionType<StyleStateChangeEventData, Style, UiContext, UiEventArgs>;
pub type UiStyleComponentStateTransitions = ComponentStateTransitions<PropagateDisplay>;
pub type SelectOptionsStateTransitions = ComponentStateTransitions<SelectOptions>;

pub type ComponentStateTransitions<TransitionGroupT> = UiEntityComponentStateTransitions<StyleStateChangeEventData, Style, UiComponentState, UiContext, UiEventArgs, TransitionGroupT>;

pub type UiSelectedComponentStateTransitions = UiEntityComponentStateTransitions<StyleStateChangeEventData, Style, UiComponentState, UiContext, UiEventArgs, SelectOptions>;
pub type UiSelectedComponentStateTransition = EntityComponentStateTransition<StyleStateChangeEventData, Style, UiComponentState, UiContext, UiEventArgs, SelectOptions>;
pub type UiSelectedComponentStateTransitionsQuery<'a> = StateTransitionsQuery<'a, Style, StyleStateChangeEventData, UiComponentState, UiContext, UiEventArgs, SelectOptions>;

pub type DraggableStateChangeRetriever = StateChangeActionTypeStateRetriever<
    DraggableUiComponentFilter, DraggableUiComponentIxnFilter, Style,
    UiContext, UiEventArgs, StyleStateChangeEventData, UiComponentState, PropagateDraggable>;

pub type ScrollableStateChangeRetriever = StateChangeActionTypeStateRetriever<
    ScrollableUiComponentFilter, ScrollableIxnFilterQuery, Style, UiContext,
    UiEventArgs, StyleStateChangeEventData, UiComponentState, PropagateScrollable>;

pub type ClickEvents = StateChangeActionTypeStateRetriever<
    UiComponentStyleFilter, UiComponentStyleIxnFilter, Style,
    UiContext, UiEventArgs, StyleStateChangeEventData, UiComponentState, PropagateDisplay>;

pub type ClickSelectionEventRetriever = StateChangeActionTypeStateRetriever<
    UiComponentStyleFilter, UiComponentStyleIxnFilter, Style,
    UiContext, UiEventArgs, StyleStateChangeEventData, UiComponentState, SelectOptions>;

pub type UiComponentEventDescriptor = EventDescriptor<StyleStateChangeEventData, UiEventArgs, Style>;


pub type UiStyleEntityComponentStateTransitions = UiEntityComponentStateTransitions<
    StyleStateChangeEventData, Style,
    UiComponentState, UiContext,
    UiEventArgs,
    PropagateDisplay
>;

