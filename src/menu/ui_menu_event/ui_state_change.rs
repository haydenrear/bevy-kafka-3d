use bevy::prelude::{BackgroundColor, Button, Changed, Color, Component, Display, Entity, info, Interaction, Query, ResMut, Resource, Size, Style, Visibility, With};
use bevy::ui::UiRect;
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::math::Vec2;
use bevy::input::mouse::MouseScrollUnit;
use crate::event::event_descriptor::{EventArgs, EventData};
use crate::event::event_state::{ComponentChangeEventData, Context, StyleStateChangeEventData, Update};
use crate::menu::{Menu, MetricsConfigurationOption, UiComponent};
use crate::menu::ui_menu_event::change_style::DoChange;
use crate::menu::ui_menu_event::next_action::{Matches, UiComponentState, VisibilityIdentifier};
use crate::menu::ui_menu_event::type_alias::event_reader_writer::{DraggableUiComponentFilter, DraggableUiComponentIxnFilter, RaycastFilter, RaycastIxnFilter, ScrollableIxnFilterQuery, ScrollableUiComponentFilter, UiComponentStyleFilter, UiComponentStyleIxnFilter, VisibleFilter, VisibleIxnFilter};
use crate::menu::ui_menu_event::type_alias::state_change_action_retriever::{ChangeVisibleEventRetriever, ClickEvents, ClickSelectionEventRetriever, DraggableStateChangeRetriever, ScrollableStateChangeRetriever};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiEventArgs;

/// Contains the state data needed in order to generate the UIEvents from the state change required.
#[derive(Clone, Debug)]
pub enum UiClickStateChange {
    ChangeColor {
        entity: Entity,
        update_display: Update<BackgroundColor>,
    },
    ChangeDisplay {
        entity: Entity,
        update_display: Update<Display>,
    },
    ChangeSize {
        entity: Entity,
        update_display: Update<Size>,
    },
    Slider {
        entity: Entity,
        update_scroll: Update<UiRect>
    },
    ChangeVisible {
        entity: Entity,
        adviser_component: Entity,
        update_component: Visibility
    },
    None,
}

pub trait StateChangeMachine<ComponentT, Ctx: Context, EventArgsT: EventArgs>: Send + Sync + 'static + EventData {
    fn state_machine_event(
        &self,
        starting: &ComponentT,
        style_context: &mut ResMut<Ctx>,
        entity: Entity
    ) -> Option<EventArgsT>;
}

impl StateChangeMachine<Visibility, UiContext, UiEventArgs> for ComponentChangeEventData {
    fn state_machine_event(&self, starting: &Visibility, style_context: &mut ResMut<UiContext>, entity: Entity) -> Option<UiEventArgs> {
        if let ComponentChangeEventData::ChangeVisible{ to_change, adviser_component} = self {
            info!("Creating change visible event with: {:?}", to_change);
            if starting == Visibility::Visible {
                return Some(UiEventArgs::Event(UiClickStateChange::ChangeVisible {
                    entity: *to_change,  update_component: Visibility::Hidden, adviser_component: *adviser_component
                }));
            } else if starting == Visibility::Hidden {
                return Some(UiEventArgs::Event(UiClickStateChange::ChangeVisible {
                    entity: *to_change,  update_component: Visibility::Visible, adviser_component: *adviser_component
                }));
            } else if starting == Visibility::Inherited {
                return Some(UiEventArgs::Event(UiClickStateChange::ChangeVisible {
                    entity: *to_change,  update_component: Visibility::Visible, adviser_component: *adviser_component
                }));
            }
        }
        None
    }
}

impl StateChangeMachine<Style, UiContext, UiEventArgs> for StyleStateChangeEventData {
    fn state_machine_event(&self, starting: &Style, style_context: &mut ResMut<UiContext>, entity: Entity) -> Option<UiEventArgs> {
        if let StyleStateChangeEventData::ChangeComponentStyle(change_style) = self {
            return change_style.do_change(starting, entity, style_context);
        }
        None
    }
}


pub fn hover_event(
    mut query: Query<(&mut Style, &mut BackgroundColor, &Interaction), (With<UiComponent>, With<Button>, Changed<Interaction>)>,
) {
    for (_, mut color, interaction) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                color.0 = Color::BLUE;
            }
            Interaction::Hovered => {
                color.0 = Color::YELLOW;
            }
            Interaction::None => {
                // color.0 = Color::GREEN;
            }
        }
    }
}

#[derive(Resource, Debug)]
pub struct GlobalState
{
    pub(crate) cursor_pos: Vec2,
    pub(crate) cursor_delta: Vec2,
    pub(crate) click_hover_ui: Option<Entity>,
    pub(crate) hover_ui: Option<Entity>,
    pub(crate) scroll_wheel_delta: Vec2,
    pub(crate) wheel_units: Option<MouseScrollUnit>
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            cursor_pos: Default::default(),
            cursor_delta: Default::default(),
            click_hover_ui: None,
            hover_ui: None,
            scroll_wheel_delta: Default::default(),
            wheel_units: None,
        }
    }
}

pub trait UpdateGlobalState<SELF, IXN>: Resource + Send + Sync
where SELF: ReadOnlyWorldQuery,
      IXN: ReadOnlyWorldQuery
{
    fn update_wheel(resource: &mut GlobalState, event: Vec2, wheel_units: Option<MouseScrollUnit>) {
        let mut prev: Vec2 = Vec2::new(event.x, event.y);
        std::mem::swap(&mut prev, &mut resource.scroll_wheel_delta);
        resource.wheel_units = wheel_units;
        if prev != Vec2::ZERO {
            let delta = resource.cursor_pos - prev;
            resource.scroll_wheel_delta = delta;
        }
    }

    fn update_cursor(resource: &mut GlobalState, cursor_pos: Vec2) {
        let mut prev: Vec2 = cursor_pos;
        std::mem::swap(&mut prev, &mut resource.cursor_pos);
        if prev != Vec2::ZERO {
            let delta = resource.cursor_pos - prev;
            resource.cursor_delta = delta;
        }
    }

    fn update_hover_ui(resource: &mut GlobalState, hover_ui: Option<Entity>) {
        resource.hover_ui = hover_ui;
    }

    fn update_click_hover_ui(resource: &mut GlobalState, hover_ui: Option<Entity>) {
        resource.hover_ui = hover_ui;
    }

    fn click_hover_ui(resources: &mut GlobalState) -> bool {
        resources.click_hover_ui.is_some()
    }

    fn hover_ui(resources: &mut GlobalState) -> bool {
        resources.hover_ui.is_some()
    }

}

impl UpdateGlobalState<UiComponentStyleFilter, UiComponentStyleIxnFilter>
for ClickEvents {}

impl UpdateGlobalState<UiComponentStyleFilter, UiComponentStyleIxnFilter>
for ClickSelectionEventRetriever {}

impl UpdateGlobalState<RaycastFilter, RaycastIxnFilter>
for ClickSelectionEventRetriever {}

impl<T: ChangeVisible> UpdateGlobalState<VisibleFilter<T>, VisibleIxnFilter<T>>
for ChangeVisibleEventRetriever<T, Visibility> {}

pub trait ChangeVisible: Component {
    fn is_visible(&self) -> bool;
}

impl<T: ChangeVisible> Matches<T> for UiComponentState {
    fn matches(&self, other: &T) -> bool {
        true
    }
}

/// This decorates the updating of the global state so that for some systems that are updating
/// on a faster timer, and not on a Changed<> timer, the hover won't be updated when truly it's
/// still hovering.
impl UpdateGlobalState<ScrollableUiComponentFilter, ScrollableIxnFilterQuery>
for ScrollableStateChangeRetriever  {
    fn update_hover_ui(resource: &mut GlobalState, hover_ui: Option<Entity>) {
        // in the event when the cursor moves from one button to another, sometimes hover gets
        // set to false and then never gets set to true again, so we let the hover event propagate
        // in this case (fixes race condition).
        if resource.hover_ui.is_none() && hover_ui.is_some() {
            resource.hover_ui = hover_ui;
        }
    }

    fn update_click_hover_ui(resource: &mut GlobalState, hover_ui: Option<Entity>) {
    }
}

/// This decorates the updating of the global state so that for some systems that are updating
/// on a faster timer, and not on a Changed<> timer, the hover won't be updated when truly it's
/// still hovering.
impl UpdateGlobalState<DraggableUiComponentFilter, DraggableUiComponentIxnFilter>
for DraggableStateChangeRetriever  {
    fn update_hover_ui(resource: &mut GlobalState, hover_ui: Option<Entity>) {
    }

    fn update_click_hover_ui(resource: &mut GlobalState, hover_ui: Option<Entity>) {
    }
}
