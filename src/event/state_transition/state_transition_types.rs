use bevy::prelude::{Entity, Style, Visibility, With};
use bevy::hierarchy::{Children, Parent};
use crate::cursor_adapter::PickableComponent;
use crate::event::event_state::{ComponentChangeEventData, StyleStateChangeEventData};
use crate::event::state_transition::parent_child_queries::{CreateMenuQueries, StyleUiComponentQueries, VisibilityComponentQueries};
use crate::event::state_transition::state_transitions_system::InsertStateTransitions;
use crate::graph::GraphingMetricsResource;
use crate::menu::config_menu_event::interaction_config_event_writer::{GraphMenuResultBuilder, NetworkMenuResultBuilder};
use crate::menu::ui_menu_event::next_action::UiComponentState;
use crate::menu::ui_menu_event::transition_groups::{PropagateCreateMenu, PropagateDisplay, PropagateSelect, PropagateVisible};
use crate::menu::{Menu, MetricsConfigurationOption, UiComponent};
use crate::menu::graphing_menu::graph_menu::{ChangeGraphingMenu, GraphMenuPotential};
use crate::menu::ui_menu_event::ui_state_change::ChangeVisible;
use crate::pickable_events::PickableComponentState;
use crate::ui_components::menu_components::BuildMenuResult;
use crate::ui_components::menu_components::menu_options::dropdown_menu_option::DropdownMenuOptionResult;
use crate::ui_components::menu_components::menu_types::base_menu::BuildBaseMenuResult;
use crate::ui_components::menu_components::menu_types::collapsable_menu::DrawCollapsableMenuResult;
use crate::ui_components::menu_components::menu_types::dropdown_menu::DrawDropdownMenuResult;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

pub(crate) type EntityQueryType<'a, TransitionGroupT, ComponentT, ComponentTypeT> = (Entity, &'a ComponentTypeT, &'a ComponentT, &'a TransitionGroupT);
pub(crate) type EntityFilterType<TransitionGroupT, ComponentT, ComponentTypeT> = (With<ComponentTypeT>, With<ComponentT>, With<TransitionGroupT>);
pub(crate) type ChildQueryType<'a, TransitionGroupT, ComponentT, ComponentTypeT> = (Entity, &'a ComponentTypeT, &'a Children, &'a ComponentT, &'a TransitionGroupT);
pub(crate) type ChildFilterType<TransitionGroupT, ComponentT, ComponentTypeT> = (With<ComponentTypeT>, With<Children>, With<ComponentT>, With<TransitionGroupT>);
pub(crate) type ParentQueryType<'a, TransitionGroupT, ComponentT, ComponentTypeT> = (Entity, &'a ComponentTypeT, &'a Parent, &'a ComponentT, &'a TransitionGroupT);
pub(crate) type ParentFilterType<TransitionGroupT, ComponentT, ComponentTypeT> = (With<ComponentTypeT>, With<Parent>, With<ComponentT>, With<TransitionGroupT>);

pub(crate) type UiEntityQuery<'a, TransitionGroupT, ComponentT> = EntityQueryType<'a, TransitionGroupT, ComponentT, UiComponent>;
pub(crate) type UiEntityFilter<TransitionGroupT, ComponentT> = EntityFilterType< TransitionGroupT, ComponentT, UiComponent>;
pub(crate) type UiChildQuery<'a, TransitionGroupT, ComponentT> = ChildQueryType<'a, TransitionGroupT, ComponentT, UiComponent>;
pub(crate) type UiChildFilter<TransitionGroupT, ComponentT> = ChildFilterType< TransitionGroupT, ComponentT, UiComponent>;
pub(crate) type UiParentQuery<'a, TransitionGroupT, ComponentT> = ParentQueryType<'a, TransitionGroupT, ComponentT, UiComponent>;
pub(crate) type UiParentFilter<TransitionGroupT, ComponentT> = ParentFilterType< TransitionGroupT, ComponentT, UiComponent>;

pub struct InsertCollapsableDisplayTransitions;

impl InsertStateTransitions<
    '_,
    PropagateDisplay,
    DrawCollapsableMenuResult,
    BuildMenuResult,
    StyleStateChangeEventData,
    StyleUiComponentQueries<PropagateDisplay>,
    UiComponent,
    Style,
    UiComponentState
>
for InsertCollapsableDisplayTransitions {}

pub struct InsertSelectStateTransitions;

impl InsertStateTransitions<
    '_,
    PropagateSelect,
    DropdownMenuOptionResult,
    BuildMenuResult,
    StyleStateChangeEventData,
    StyleUiComponentQueries<PropagateSelect>,
    UiComponent,
    Style,
    UiComponentState
>
for InsertSelectStateTransitions {}

pub struct InsertDropdownDisplayTransitions;

impl InsertStateTransitions<
    '_,
    PropagateDisplay,
    DrawDropdownMenuResult,
    BuildMenuResult,
    StyleStateChangeEventData,
    StyleUiComponentQueries<PropagateDisplay>,
    UiComponent,
    Style,
    UiComponentState
>
for InsertDropdownDisplayTransitions {}

pub struct InsertBaseMenuDisplayTransitions;

impl InsertStateTransitions<
    '_,
    PropagateDisplay,
    BuildBaseMenuResult,
    BuildMenuResult,
    StyleStateChangeEventData,
    StyleUiComponentQueries<PropagateDisplay>,
    UiComponent,
    Style,
    UiComponentState
>
for InsertBaseMenuDisplayTransitions {}

pub struct InsertVisibleGraphStateTransitions;

impl InsertStateTransitions<
    '_,
    PropagateVisible,
    GraphMenuResultBuilder,
    GraphMenuResultBuilder,
    ComponentChangeEventData,
    VisibilityComponentQueries<MetricsConfigurationOption<Menu>>,
    UiComponent,
    MetricsConfigurationOption<Menu>,
    UiComponentState,
    Visibility
>
for InsertVisibleGraphStateTransitions {}

pub struct InsertVisibleNetworkStateTransitions;

impl InsertStateTransitions<
    '_,
    PropagateVisible,
    NetworkMenuResultBuilder,
    NetworkMenuResultBuilder,
    ComponentChangeEventData,
    VisibilityComponentQueries<MetricsConfigurationOption<Menu>>,
    UiComponent,
    MetricsConfigurationOption<Menu>,
    UiComponentState,
    Visibility
>
for InsertVisibleNetworkStateTransitions {}

pub(crate) type PickableEntityQuery<'a, TransitionGroupT, ComponentT> = EntityQueryType<'a, TransitionGroupT, ComponentT, PickableComponent>;
pub(crate) type PickableEntityFilter<TransitionGroupT, ComponentT> = EntityFilterType< TransitionGroupT, ComponentT, PickableComponent>;
pub(crate) type PickableChildQuery<'a, TransitionGroupT, ComponentT> = ChildQueryType<'a, TransitionGroupT, ComponentT, PickableComponent>;
pub(crate) type PickableChildFilter<TransitionGroupT, ComponentT> = ChildFilterType< TransitionGroupT, ComponentT, PickableComponent>;
pub(crate) type PickableParentQuery<'a, TransitionGroupT, ComponentT> = ParentQueryType<'a, TransitionGroupT, ComponentT, PickableComponent>;
pub(crate) type PickableParentFilter<TransitionGroupT, ComponentT> = ParentFilterType< TransitionGroupT, ComponentT, PickableComponent>;

pub struct InsertGraphMenuStateTransitions;

impl InsertStateTransitions<
    '_,
    PropagateCreateMenu,
    GraphingMetricsResource,
    GraphingMetricsResource,
    ComponentChangeEventData,
    CreateMenuQueries,
    PickableComponent,
    GraphMenuPotential,
    PickableComponentState,
    ChangeGraphingMenu
>
for InsertGraphMenuStateTransitions {}
