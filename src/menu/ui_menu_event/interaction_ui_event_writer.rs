use std::marker::PhantomData;
use std::os::macos::raw::stat;
use bevy::prelude::{Button, Changed, Commands, Component, Display, Entity, EventWriter, Interaction, Query, ResMut, Resource, Style, Vec2, Visibility, With};
use bevy::hierarchy::{Children, Parent};
use bevy::utils::{HashMap, HashSet};
use bevy::log::info;
use bevy::ecs::query::{QueryEntityError, ReadOnlyWorldQuery, WorldQuery};
use bevy::input::mouse::MouseScrollUnit;
use bevy::ui::Size;
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_propagation::{ChangePropagation, PropagateComponentEvent, Relationship};
use crate::event::event_actions::{ClickWriteEvents, RetrieveState};
use crate::event::event_state::{Context, StateChange};
use crate::menu::{DraggableComponent, ScrollableComponent, UiComponent};
use crate::menu::ui_menu_event::change_style::{ChangeStyleTypes};
use crate::menu::ui_menu_event::next_action::UiComponentState;
use crate::menu::ui_menu_event::style_context::StyleContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{EntityComponentStateTransition, PropagationQuery, PropagationQueryFilter, ScrollableIxnFilterQuery, ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter, StateChangeActionType, UiComponentStateFilter, UiComponentStateTransition, UiComponentStateTransitions, UiComponentStateTransitionsQuery, UiEventArgs};
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

#[derive(Resource, Debug)]
pub struct StateChangeActionTypeStateRetriever<SELF: ReadOnlyWorldQuery, IXN: ReadOnlyWorldQuery> ( PhantomData<SELF> , PhantomData<IXN>);

#[derive(Resource, Debug)]
pub struct GlobalState
{
    pub(crate) cursor_pos: Vec2,
    pub(crate) cursor_delta: Vec2,
    pub(crate) click_hover_ui: bool,
    pub(crate) hover_ui: bool,
    pub(crate) scroll_wheel_delta: Vec2,
    pub(crate) wheel_units: Option<MouseScrollUnit>
}
impl Default for GlobalState {
    fn default() -> Self {
        Self {
            cursor_pos: Default::default(),
            cursor_delta: Default::default(),
            click_hover_ui: false,
            hover_ui: false,
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
        info!("{:?} is scroll wheel delta", &resource.scroll_wheel_delta);
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

    fn update_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
        resource.hover_ui = hover_ui;
    }

    fn update_click_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
        resource.hover_ui = hover_ui;
    }
    fn click_hover_ui(resources: &mut GlobalState) -> bool {
        resources.click_hover_ui
    }

}

impl UpdateGlobalState<UiComponentStyleFilter, UiComponentStyleIxnFilter>
for StateChangeActionTypeStateRetriever<UiComponentStyleFilter, UiComponentStyleIxnFilter> {}

impl UpdateGlobalState<ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter>
for StateChangeActionTypeStateRetriever<ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter>   {
    fn update_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
    }

    fn update_click_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
    }
}

impl UpdateGlobalState<DraggableUiComponentFilter, DraggableUiComponentIxnFilter> for StateChangeActionTypeStateRetriever<DraggableUiComponentFilter, DraggableUiComponentIxnFilter>  {
    fn update_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
    }

    fn update_click_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
    }
}

macro_rules! global_defaults {
    ($($ty1:ty, $ty2:ty),*) => {
        use crate::menu::config_menu_event::interaction_config_event_writer::MetricsSelfQueryFilter;
        use crate::menu::Menu;
        use crate::menu::config_menu_event::interaction_config_event_writer::MetricsSelfIxnQueryFilter;
        use crate::menu::ui_menu_event::ui_menu_event_plugin::UiComponentStyleFilter;
        use crate::menu::ui_menu_event::ui_menu_event_plugin::UiComponentStyleIxnFilter;
        use crate::menu::ui_menu_event::ui_menu_event_plugin::DraggableUiComponentFilter;
        use crate::menu::ui_menu_event::ui_menu_event_plugin::DraggableUiComponentIxnFilter;
        $(
            impl Default for StateChangeActionTypeStateRetriever<$ty1, $ty2>  {
                fn default() -> Self {
                    Self(PhantomData::default(), PhantomData::default())
                }
            }
        )*
    }
}

global_defaults!(
    MetricsSelfQueryFilter<Menu>, MetricsSelfIxnQueryFilter<Menu>,
    UiComponentStyleFilter, UiComponentStyleIxnFilter,
    DraggableUiComponentFilter, DraggableUiComponentIxnFilter,
    ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter
);

impl ClickWriteEvents<
    StateChangeActionTypeStateRetriever<UiComponentStyleFilter, UiComponentStyleIxnFilter>,
                UiEventArgs, StateChangeActionType, Style, StyleContext,
                // self query
                UiComponentStateTransitionsQuery<'_>,
                // self filter
                UiComponentStyleFilter,
                // parent query
                PropagationQuery<'_>,
                // parent filter
                PropagationQueryFilter<'_>,
                // interaction filter
                UiComponentStyleIxnFilter
> for StateChangeActionTypeStateRetriever<UiComponentStyleFilter, UiComponentStyleIxnFilter> {
}

#[derive(Default, Resource, Debug)]
pub struct DragEvents;

impl ClickWriteEvents<
    StateChangeActionTypeStateRetriever<
        DraggableUiComponentFilter,
        DraggableUiComponentIxnFilter
    >,
    UiEventArgs, StateChangeActionType, Style, StyleContext,
    // self query
    UiComponentStateTransitionsQuery<'_>,
    // self filter
    DraggableUiComponentFilter,
    // parent query
    PropagationQuery<'_>,
    // parent filter
    PropagationQueryFilter<'_>,
    // interaction filter
    DraggableUiComponentIxnFilter
> for DragEvents {
}

#[derive(Default, Resource, Debug)]
pub struct ScrollEvents;

impl ClickWriteEvents<
    StateChangeActionTypeStateRetriever<
        ScrollableUiComponentFilter,
        ScrollableUiComponentIxnFilter
    >,
    UiEventArgs, StateChangeActionType, Style, StyleContext,
    // self query
    UiComponentStateTransitionsQuery<'_>,
    // self filter
    ScrollableUiComponentFilter,
    // parent query
    PropagationQuery<'_>,
    // parent filter
    PropagationQueryFilter<'_>,
    // interaction filter
    ScrollableIxnFilterQuery
> for ScrollEvents {
}

impl <SELF: ReadOnlyWorldQuery + Send + Sync + 'static, IXN: ReadOnlyWorldQuery + Send + Sync + 'static> RetrieveState<
    UiEventArgs, StateChangeActionType, Style, StyleContext,
    UiComponentStateTransitionsQuery<'_>,
    PropagationQuery<'_>,
    SELF,
    PropagationQueryFilter<'_>,
>
for StateChangeActionTypeStateRetriever<
    SELF,
    IXN
>
{
    fn create_event(
        mut commands: &mut Commands,
        entity: Entity,
        mut style_context: &mut ResMut<StyleContext>,
        entity_query: &Query<
            UiComponentStateTransitionsQuery<'_>,
            SELF
        >,
        propagation_query: &Query<
            PropagationQuery<'_>,
            PropagationQueryFilter
        >
    ) -> (Vec<EventDescriptor<StateChangeActionType, UiEventArgs, Style>>, Vec<PropagateComponentEvent>)
    {
        let mut event_descriptors = vec![];
        let mut propagate_events = vec![];

        Self::create_ui_event(&entity_query, &propagation_query, &mut style_context, entity)
            .into_iter()
            .for_each(|prop| {
                info!("Sending event: {:?}.", prop);
                event_descriptors.push(prop);
            });

        (event_descriptors, propagate_events)

    }
}

impl<SELF, IXN> StateChangeActionTypeStateRetriever<SELF, IXN>
    where
        SELF: ReadOnlyWorldQuery + Send + Sync + 'static,
        IXN: ReadOnlyWorldQuery + Send + Sync + 'static
{
    fn create_ui_event(
        entity_query: &Query<
            UiComponentStateTransitionsQuery<'_>,
            SELF
        >,
        propagation_query: &Query<PropagationQuery<'_>, PropagationQueryFilter>,
        mut style_context: &mut ResMut<StyleContext>,
        entity: Entity
    ) -> Vec<EventDescriptor<StateChangeActionType, UiEventArgs, Style>> {
        entity_query.get(entity)
            .iter()
            .flat_map(|entity| {
                entity.4.transitions.iter()
                    .map(|transition| (entity.0, entity.1, entity.2, entity.3, transition))
            })
            .flat_map(|entity| {

                info!("{:?} is state transition.", entity.4);
                let mut descriptors = vec![];

                if !entity.4.filter_state.matches(entity.2) {
                    return vec![];
                }

                for (related_entity, _, state_change_action_type) in entity.4.entity_to_change.states.iter() {
                    info!("{:?} is state change action type.", state_change_action_type);
                    let (_, related_style, _) = propagation_query.get(*related_entity).unwrap();

                    if !entity.4.current_state_filter.matches(related_style) {
                        info!("Did not match.");
                        continue;
                    }

                    Self::create_add_event(&mut style_context, &mut descriptors, state_change_action_type, &related_style, *related_entity);
                }

                descriptors
            })
            .collect()

    }

    fn create_add_event(
        mut style_context: &mut ResMut<StyleContext>,
        mut descriptors: &mut Vec<EventDescriptor<StateChangeActionType, UiEventArgs, Style>>,
        state_change_action_type: &StateChangeActionType,
        related_style: &&Style,
        entity: Entity
    ) {
        info!("Creating add event.");
        match state_change_action_type {
            StateChangeActionType::Hover(_) => {
                None
            }
            StateChangeActionType::Clicked(clicked)
            | StateChangeActionType::Dragged(clicked) => {
                info!("Creating click drag event.");
                clicked.get_ui_event(&related_style, &mut style_context, entity)
                    .map(|args| {
                        info!("Created ui event args: {:?}.", &args);
                        EventDescriptor {
                            component: PhantomData::default(),
                            event_data: state_change_action_type.clone(),
                            event_args: args,
                        }
                    })
            }
        }.map(|descriptor| {
            descriptors.push(descriptor);
        });
    }
}

