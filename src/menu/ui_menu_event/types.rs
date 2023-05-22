use bevy::prelude::{Button, Changed, Entity, Interaction, Resource, Style, Visibility, With};
use crate::cursor_adapter::RayCastActionable;
use crate::event::event_descriptor::EventDescriptor;
use crate::event::event_state::{ComponentChangeEventData, StyleStateChangeEventData};
use crate::menu::ui_menu_event::interaction_ui_event_writer::{StateChangeActionTypeStateRetriever};
use crate::menu::ui_menu_event::next_action::{UiComponentState};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{EntityComponentStateTransition, PropagateDisplay, PropagateDraggable, PropagateRaycast, PropagateScrollable, PropagateSelect, PropagateVisible, StateChangeActionType, UiEntityComponentStateTransitions, UiEventArgs};
use crate::menu::{DraggableComponent, Menu, MetricsConfigurationOption, ScrollableComponent, UiComponent};
use crate::menu::ui_menu_event::interaction_ui_event_reader::ComponentChangeEventReader;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

pub type UiComponentStyleFilter = (With<UiComponent>, With<Style>);
pub type UiComponentStyleIxnFilter = (With<UiComponent>, With<Button>, Changed<Interaction>);

pub type RaycastFilter = (With<RayCastActionable>);
pub type RaycastIxnFilter = (With<RayCastActionable>);

pub type VisibleFilter<T> = (With<T>);
pub type VisibleIxnFilter<T> = (With<T>, With<Button>, Changed<Interaction>);

pub type DraggableUiComponentFilter = (With<UiComponent>, With<Style>, With<DraggableComponent>);
pub type DraggableUiComponentIxnFilter = (With<UiComponent>, With<Button>, With<DraggableComponent>);

pub type ScrollableUiComponentFilter = (With<UiComponent>, With<Style>, With<ScrollableComponent>);
pub type ScrollableIxnFilterQuery = (With<UiComponent>, With<ScrollableComponent>);

pub type PropagationQueryFilter<C> = (With<C>);
pub type PropagationQuery<'a, C> = (Entity, &'a C, &'a UiIdentifiableComponent);

pub type StateTransitionsQuery<'a, StateComponentT, ChangeComponentT, StateMachineT, UpdateComponentMatchesT, FilterMatches, Ctx, EventArgsT, TransitionGroupT> =
(Entity,
 &'a UiComponent,
 &'a StateComponentT,
 &'a UiIdentifiableComponent,
 &'a UiEntityComponentStateTransitions<
     StateMachineT,
     StateComponentT,
     FilterMatches,
     ChangeComponentT,
     UpdateComponentMatchesT,
     Ctx,
     EventArgsT,
     TransitionGroupT
 >);

pub type StyleUiComponentStateTransitionsQuery<'a> = StateTransitionsQuery<'a, Style, Style, StyleStateChangeEventData, UiComponentState, UiComponentState, UiContext, UiEventArgs, PropagateDisplay>;
pub type StylePropagationQuery<'a> = PropagationQuery<'a, Style>;
pub type StylePropagationQueryFilter = PropagationQueryFilter<Style>;

pub type PropagateStateTransitionsQuery<'a, TransitionGroupT> = StateTransitionsQuery<'a, Style, Style, StyleStateChangeEventData, UiComponentState, UiComponentState, UiContext, UiEventArgs, TransitionGroupT>;

pub type UiStateChange<C, S> = StateChangeActionType<S, C, UiContext, UiEventArgs>;
pub type StyleStateChange = StateChangeActionType<StyleStateChangeEventData, Style, UiContext, UiEventArgs>;
pub type UiStyleComponentStateTransitions = ComponentStateTransitions<PropagateDisplay, StyleStateChangeEventData, Style, UiComponentState, Style, UiComponentState>;
pub type SelectOptionsStateTransitions = ComponentStateTransitions<PropagateSelect, StyleStateChangeEventData, Style, UiComponentState, Style, UiComponentState>;
pub type DraggableStateTransitions = ComponentStateTransitions<PropagateDraggable, StyleStateChangeEventData, Style, UiComponentState, Style, UiComponentState>;

pub type VisibilityStateTransitions = UiEntityComponentStateTransitions<ComponentChangeEventData, MetricsConfigurationOption<Menu>, UiComponentState, Visibility, UiComponentState, UiContext, UiEventArgs, PropagateVisible>;

pub type ComponentStateTransitions<TransitionGroupT, EventData, StateComponentT, FilterMatchesT, UpdateComponentT, UpdateMatchesT> = UiEntityComponentStateTransitions<EventData, StateComponentT, FilterMatchesT, UpdateComponentT, UpdateMatchesT, UiContext, UiEventArgs, TransitionGroupT>;

pub type UiSelectedComponentStateTransitions = UiEntityComponentStateTransitions<StyleStateChangeEventData, Style, UiComponentState, Style, UiComponentState, UiContext, UiEventArgs, PropagateSelect>;
pub type UiSelectedComponentStateTransition = EntityComponentStateTransition<StyleStateChangeEventData, Style, UiComponentState, Style, UiComponentState, UiContext, UiEventArgs, PropagateSelect>;
pub type UiSelectedComponentStateTransitionsQuery<'a> = StateTransitionsQuery<'a, Style, Style, StyleStateChangeEventData, UiComponentState, UiComponentState, UiContext, UiEventArgs, PropagateSelect>;

pub type VisibleComponentStateTransitionsQuery<'a, ComponentStateT, ComponentChangeT> = StateTransitionsQuery<'a, ComponentStateT, ComponentChangeT, ComponentChangeEventData, UiComponentState, UiComponentState, UiContext, UiEventArgs, PropagateVisible>;

pub type DraggableStateChangeRetriever = StateChangeActionTypeStateRetriever<
    DraggableUiComponentFilter, DraggableUiComponentIxnFilter, Style, Style,
    UiContext, UiEventArgs, StyleStateChangeEventData, UiComponentState, UiComponentState, PropagateDraggable>;

pub type ScrollableStateChangeRetriever = StateChangeActionTypeStateRetriever<
    ScrollableUiComponentFilter, ScrollableIxnFilterQuery, Style, Style, UiContext,
    UiEventArgs, StyleStateChangeEventData, UiComponentState, UiComponentState, PropagateScrollable>;

pub type ClickEvents = StateChangeActionTypeStateRetriever<
    UiComponentStyleFilter, UiComponentStyleIxnFilter, Style, Style,
    UiContext, UiEventArgs, StyleStateChangeEventData, UiComponentState, UiComponentState, PropagateDisplay>;


pub type ClickSelectionEventRetriever = StateChangeActionTypeStateRetriever<
    UiComponentStyleFilter, UiComponentStyleIxnFilter, Style, Style,
    UiContext, UiEventArgs, StyleStateChangeEventData, UiComponentState, UiComponentState, PropagateSelect>;

pub type RaycastActionableEventRetriever = StateChangeActionTypeStateRetriever<
    RaycastFilter, RaycastIxnFilter, RayCastActionable, RayCastActionable,
    UiContext, UiEventArgs, ComponentChangeEventData, UiComponentState, UiComponentState, PropagateRaycast>;

pub type ChangeVisibleEventRetriever<StateComponentT, ChangeComponentT> = StateChangeActionTypeStateRetriever<
    VisibleFilter<StateComponentT>, VisibleIxnFilter<StateComponentT>, StateComponentT, ChangeComponentT,
    UiContext, UiEventArgs, ComponentChangeEventData, UiComponentState, UiComponentState, PropagateVisible>;

pub type UiComponentEventDescriptor = EventDescriptor<StyleStateChangeEventData, UiEventArgs, Style>;


pub type UiStyleEntityComponentStateTransitions = UiEntityComponentStateTransitions<
    StyleStateChangeEventData, Style,
    UiComponentState, Style, UiComponentState, UiContext,
    UiEventArgs,
    PropagateDisplay
>;

pub type VisibilityComponentChangeEventReader = ComponentChangeEventReader<Visibility, Visibility, UiContext>;
