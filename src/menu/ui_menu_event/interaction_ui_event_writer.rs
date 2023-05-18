use std::fmt::Debug;
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
use crate::event::event_state::{Context, StyleStateChangeEventData};
use crate::menu::{DraggableComponent, ScrollableComponent, UiComponent};
use crate::menu::ui_menu_event::change_style::{ChangeStyleTypes};
use crate::menu::ui_menu_event::next_action::{Matches, UiComponentState};
use crate::menu::ui_menu_event::style_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{DraggableStateChangeRetriever, EntityComponentStateTransition, PropagationQuery, PropagationQueryFilter, ScrollableIxnFilterQuery, ScrollableStateChangeRetriever, ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter, StateChangeActionType, StyleStateChange, UiComponentStateChangeRetriever, UiComponentStateFilter, UiComponentStateTransition, UiComponentStateTransitions, UiComponentStateTransitionsQuery, UiEventArgs, UiStateChange};
use crate::menu::ui_menu_event::ui_state_change::StateChangeMachine;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

#[derive(Resource, Debug)]
pub struct StateChangeActionTypeStateRetriever<SELF, IXN, S, Ctx, Args, StateMachine, MatchesT> (
    PhantomData<SELF> ,
    PhantomData<IXN>,
    PhantomData<S>,
    PhantomData<Ctx>,
    PhantomData<Args>,
    PhantomData<StateMachine>,
    PhantomData<MatchesT>
)
where
    SELF: ReadOnlyWorldQuery,
    IXN: ReadOnlyWorldQuery,
    S: Component,
    Ctx: Context,
    Args: EventArgs,
    StateMachine: StateChangeMachine<S, Ctx, Args>,
    MatchesT: Matches<S>;

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
for UiComponentStateChangeRetriever {}

impl UpdateGlobalState<ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter>
for ScrollableStateChangeRetriever  {
    fn update_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
    }

    fn update_click_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
    }
}

impl UpdateGlobalState<DraggableUiComponentFilter, DraggableUiComponentIxnFilter>
for DraggableStateChangeRetriever  {
    fn update_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
    }

    fn update_click_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
    }
}

macro_rules! global_defaults {
    ($($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty),*) => {
        use crate::menu::Menu;
        use crate::menu::ui_menu_event::ui_menu_event_plugin::UiComponentStyleFilter;
        use crate::menu::ui_menu_event::ui_menu_event_plugin::UiComponentStyleIxnFilter;
        use crate::menu::ui_menu_event::ui_menu_event_plugin::DraggableUiComponentFilter;
        use crate::menu::ui_menu_event::ui_menu_event_plugin::DraggableUiComponentIxnFilter;
        $(
            impl Default for StateChangeActionTypeStateRetriever<$ty1, $ty2, $ty3, $ty4, $ty5, StyleStateChangeEventData, UiComponentState>  {
                fn default() -> Self {
                    Self(
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default()
                    )
                }
            }
        )*
    }
}

global_defaults!(
    UiComponentStyleFilter, UiComponentStyleIxnFilter, Style, UiContext, UiEventArgs,
    DraggableUiComponentFilter, DraggableUiComponentIxnFilter, Style, UiContext, UiEventArgs,
    ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter, Style, UiContext, UiEventArgs
);

impl ClickWriteEvents<
    UiComponentStateChangeRetriever,
    UiEventArgs, StyleStateChangeEventData, Style, UiContext,
    // self query
    UiComponentStateTransitionsQuery<'_, Style, StyleStateChangeEventData, UiComponentState>,
    // self filter
    UiComponentStyleFilter,
    // parent query
    PropagationQuery<'_, Style>,
    // parent filter
    PropagationQueryFilter<Style>,
    // interaction filter
    UiComponentStyleIxnFilter
> for UiComponentStateChangeRetriever {}

#[derive(Default, Resource, Debug)]
pub struct DragEvents;

// impl ClickWriteEvents<
//     DraggableStateChangeRetriever,
//     UiEventArgs, StyleStateChange, Style, UiContext,
//     // self query
//     UiComponentStateTransitionsQuery<'_, Style>,
//     // self filter
//     DraggableUiComponentFilter,
//     // parent query
//     PropagationQuery<'_, Style>,
//     // parent filter
//     PropagationQueryFilter<Style>,
//     // interaction filter
//     DraggableUiComponentIxnFilter
// > for DragEvents {
// }

#[derive(Default, Resource, Debug)]
pub struct ScrollEvents;


// impl ClickWriteEvents<
//     ScrollableStateChangeRetriever,
//     UiEventArgs, StyleStateChange, Style, UiContext,
//     // self query
//     UiComponentStateTransitionsQuery<'_, Style>,
//     // self filter
//     ScrollableUiComponentFilter,
//     // parent query
//     PropagationQuery<'_, Style>,
//     // parent filter
//     PropagationQueryFilter<Style>,
//     // interaction filter
//     ScrollableIxnFilterQuery
// > for ScrollEvents {
// }

// impl <SelfFilterQuery, SelfIxnFilter, ComponentT, Ctx, EventArgsT, EventDataT> RetrieveState<
//     EventArgsT,
//     EventDataT,
//     ComponentT,
//     Ctx,
//     UiComponentStateTransitionsQuery<'_, ComponentT>,
//     PropagationQuery<'_, ComponentT>,
//     SelfFilterQuery,
//     PropagationQueryFilter<ComponentT>,
// >
// for StateChangeActionTypeStateRetriever<
//     SelfFilterQuery, SelfIxnFilter,
//     ComponentT, Ctx, EventArgsT, EventDataT
// >
// where
//     SelfIxnFilter: ReadOnlyWorldQuery,
//     SelfFilterQuery: ReadOnlyWorldQuery,
//     ComponentT: Component + Send + Sync + 'static + Debug,
//     Ctx: Context,
//     EventArgsT: EventArgs + Debug + 'static,
//     EventDataT: StateChangeMachine<ComponentT, Ctx, EventArgsT> + EventData + 'static
// {
//     fn create_event(
//         mut commands: &mut Commands,
//         entity: Entity,
//         mut style_context: &mut ResMut<Ctx>,
//         entity_query: &Query<
//             UiComponentStateTransitionsQuery<'_, ComponentT>,
//             SelfFilterQuery
//         >,
//         propagation_query: &Query<
//             PropagationQuery<'_, ComponentT>,
//             PropagationQueryFilter<ComponentT>
//         >
//     ) -> (Vec<EventDescriptor<EventDataT, EventArgsT, ComponentT>>, Vec<PropagateComponentEvent>)
//     {
//         let mut event_descriptors = vec![];
//         let mut propagate_events = vec![];
//
//         Self::create_ui_event(&entity_query, &propagation_query, &mut style_context, entity)
//             .into_iter()
//             .for_each(|prop| {
//                 event_descriptors.push(prop);
//             });
//
//         (event_descriptors, propagate_events)
//
//     }
// }
//

impl RetrieveState<
    UiEventArgs,
    StyleStateChangeEventData,
    Style,
    UiContext,
    UiComponentStateTransitionsQuery<'_, Style, StyleStateChangeEventData, UiComponentState>,
    PropagationQuery<'_, Style>,
    UiComponentStyleFilter,
    PropagationQueryFilter<Style>,
>
for UiComponentStateChangeRetriever
{
    fn create_event(
        mut commands: &mut Commands,
        entity: Entity,
        mut style_context: &mut ResMut<UiContext>,
        entity_query: &Query<
            UiComponentStateTransitionsQuery<'_, Style, StyleStateChangeEventData, UiComponentState>,
            UiComponentStyleFilter
        >,
        propagation_query: &Query<
            PropagationQuery<'_, Style>,
            PropagationQueryFilter<Style>
        >
    ) -> (Vec<EventDescriptor<StyleStateChangeEventData, UiEventArgs, Style>>, Vec<PropagateComponentEvent>)
    {
        let mut event_descriptors = vec![];
        let mut propagate_events = vec![];

        Self::create_ui_event(&entity_query, &propagation_query, &mut style_context, entity)
            .into_iter()
            .for_each(|prop| {
                event_descriptors.push(prop);
            });

        (event_descriptors, propagate_events)

    }
}

impl<SelfFilterQuery, IXN, C, StateMachine, MatchesT> StateChangeActionTypeStateRetriever<SelfFilterQuery, IXN, C, UiContext, UiEventArgs, StateMachine, MatchesT>
    where
        SelfFilterQuery: ReadOnlyWorldQuery + Send + Sync + 'static,
        IXN: ReadOnlyWorldQuery + Send + Sync + 'static,
        C: Component + Debug,
        StateMachine: StateChangeMachine<C, UiContext, UiEventArgs> + Send + Sync + EventData + 'static + Clone,
        MatchesT: Matches<C>
{
    fn create_ui_event(
        entity_query: &Query<
            UiComponentStateTransitionsQuery<'_, C, StateMachine, MatchesT>,
            SelfFilterQuery
        >,
        propagation_query: &Query<
            PropagationQuery<'_, C>,
            PropagationQueryFilter<C>
        >,
        mut style_context: &mut ResMut<UiContext>,
        entity: Entity
    ) -> Vec<EventDescriptor<StateMachine, UiEventArgs, C>> {
        entity_query.get(entity)
            .iter()
            .flat_map(|entity| {
                entity.4.transitions.iter()
                    .map(|transition| (entity.0, entity.1, entity.2, entity.3, transition))
            })
            .flat_map(|entity| {

                let mut descriptors = vec![];

                if !entity.4.filter_state.matches(entity.2) {
                    return vec![];
                }

                for (related_entity, _, state_change_action_type) in entity.4.entity_to_change.states.iter() {
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
        mut style_context: &mut ResMut<UiContext>,
        mut descriptors: &mut Vec<EventDescriptor<StateMachine, UiEventArgs, C>>,
        state_change_action_type: &UiStateChange<C, StateMachine>,
        related_style: &C,
        entity: Entity
    ) {
        match state_change_action_type {
            StateChangeActionType::Clicked{value, ..} => {
                value.state_machine_event(related_style, style_context, entity)
                    .map(|args| {
                        info!("Created ui event args: {:?}.", &args);
                        EventDescriptor {
                            component: PhantomData::default(),
                            event_data: value.clone(),
                            event_args: args,
                        }
                    })
                    .map(|descriptor| {
                        descriptors.push(descriptor);
                    });
            }
            _ => {

            }
        }
    }
}

