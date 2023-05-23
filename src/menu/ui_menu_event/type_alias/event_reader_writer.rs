use bevy::prelude::{Button, Changed, Entity, Interaction, Style, Visibility, With};
use crate::cursor_adapter::PickableComponent;
use crate::event::event_descriptor::EventDescriptor;
use crate::event::event_state::{ComponentChangeEventData, StyleStateChangeEventData};
use crate::menu::{DraggableComponent, ScrollableComponent, UiComponent};
use crate::menu::ui_menu_event::interaction_ui_event_reader::ComponentChangeEventReader;
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiEventArgs;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

pub type UiComponentStyleFilter = (With<UiComponent>, With<Style>);
pub type UiComponentStyleIxnFilter = (With<UiComponent>, With<Button>, Changed<Interaction>);

pub type PickableFilter = (With<PickableComponent>);
pub type PickableIxnFilter = (With<PickableComponent>);

pub type VisibleFilter<T> = (With<T>);
pub type VisibleIxnFilter<T> = (With<T>, With<Button>, Changed<Interaction>);

pub type DraggableUiComponentFilter = (With<UiComponent>, With<Style>, With<DraggableComponent>);
pub type DraggableUiComponentIxnFilter = (With<UiComponent>, With<Button>, With<DraggableComponent>);

pub type ScrollableUiComponentFilter = (With<UiComponent>, With<Style>, With<ScrollableComponent>);
pub type ScrollableIxnFilterQuery = (With<UiComponent>, With<ScrollableComponent>);

pub type PropagationQueryFilter<C> = (With<C>);
pub type PropagationQuery<'a, C> = (Entity, &'a C, &'a UiIdentifiableComponent);

pub type StylePropagationQuery<'a> = PropagationQuery<'a, Style>;
pub type StylePropagationQueryFilter = PropagationQueryFilter<Style>;

pub type UiComponentEventDescriptor = EventDescriptor<StyleStateChangeEventData, UiEventArgs, Style>;
pub type VisibilityEventDescriptor = EventDescriptor<ComponentChangeEventData, UiEventArgs, Visibility>;

pub type VisibilityComponentChangeEventReader<ChangeDisplayT> = ComponentChangeEventReader<Visibility, ChangeDisplayT, UiContext>;
