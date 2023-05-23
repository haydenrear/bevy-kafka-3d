use bevy::ecs::query::{QueryEntityError, ReadOnlyWorldQuery};
use bevy::prelude::*;
use bevy::window::CursorMoved;
use crate::camera::raycast_select::BevyPickingState;
use crate::interactions::InteractionEvent;
use bevy::input::Input;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::{Entity, EventReader, MouseButton, Query, Res, ResMut};
use bevy_mod_picking::{HoverEvent, PickingEvent, SelectionEvent};
use crate::camera::ZoomableDraggableCamera;
use crate::graph::GraphDimComponent;
use crate::menu::graphing_menu::graph_menu::GraphMenuPotential;
use crate::menu::ui_menu_event::type_alias::event_reader_writer_filter::{DraggableUiComponentIxnFilter, PickableIxnFilter, ScrollableIxnFilterQuery};

/// Will be used to adapt all events into a single InteractionEvent type, which is generic over
/// the query which is used, so that events can be filtered for the different Ui systems. Ultimately,
/// the event system needs to be used for both UI events and other event types, updating the state
/// of 3d elements and UI elements.
pub(crate) fn event_merge_propagate<InteractionFilterQueryT: ReadOnlyWorldQuery + Send + Sync + 'static>(
    mut event_writer: EventWriter<InteractionEvent<InteractionFilterQueryT>>,
    interaction_query: Query<(Entity, &Interaction), InteractionFilterQueryT>,
)
{
    let _ = interaction_query
        .into_iter()
        .for_each(|(entity, interaction)| {
            event_writer.send(InteractionEvent::UiComponentInteraction {
                event: *interaction,
                entity
            });
        });
}

pub(crate) fn propagate_scroll_events(
    mut event_writer: EventWriter<InteractionEvent<ScrollableIxnFilterQuery>>,
    mut scroll_wheel_events: EventReader<MouseWheel>
) {
    for scroll in scroll_wheel_events.iter() {
        event_writer.send(InteractionEvent::ScrollWheelEvent {
            event: scroll.clone()
        });
    }
}

pub(crate) fn propagate_drag_events(
    mut event_writer: EventWriter<InteractionEvent<DraggableUiComponentIxnFilter>>,
    mut scroll_wheel_events: EventReader<CursorMoved>
) {
    for cursor_moved in scroll_wheel_events.iter() {
        event_writer.send(InteractionEvent::CursorEvent {
            event: cursor_moved.clone()
        });
    }
}

pub trait MatchesPickingEvent {
    fn matches(picking_event: &PickingEvent, raycast_actionable: Result<(Entity, &PickableComponent), QueryEntityError>) -> bool;
}

impl MatchesPickingEvent for InteractionEvent<PickableIxnFilter<GraphMenuPotential>> {
    fn matches(picking_event: &PickingEvent, raycast_actionable: Result<(Entity, &PickableComponent), QueryEntityError>) -> bool {
        raycast_actionable
            .map(|(entity, r)| {
                info!("Had interaction with pickable component.");
                match r {
                    PickableComponent::GraphDim => true,
                    _ => false
                }
            })
            .or::<QueryEntityError>(Ok(false))
            .unwrap()
    }
}

/// When an event happens with the raycast, maybe the event will want to be included so that some
/// action can be taken. This allows interaction between the 3d and the UI event system. When the
/// nodes are selected, a menu needs to pop up.
#[derive(Component, Debug)]
pub enum PickableComponent {
    GraphDim,
    Metric
}

fn get_entity(picking_event: &PickingEvent) -> Entity {
    match picking_event {
        PickingEvent::Selection(selected) => {
            match selected {
                SelectionEvent::JustSelected(e) => {
                    *e
                }
                SelectionEvent::JustDeselected(e) => {
                    *e
                }
            }
        }
        PickingEvent::Hover(hover) => {
            match hover {
                HoverEvent::JustEntered(e) => {
                    *e
                }
                HoverEvent::JustLeft(e) => {
                    *e
                }
            }
        }
        PickingEvent::Clicked(clicked) => {
            *clicked
        }
    }
}


macro_rules! ray_cast_events_system {
    () => {
        pub(crate) fn calculate_picks(
            mut raycast_source: EventReader<PickingEvent>,
            raycast_actionable: Query<(Entity, &RayCastActionable), (With<RayCastActionable>)>,
            mut intersected: ResMut<BevyPickingState>,
            cam: Res<ZoomableDraggableCamera>,
            mouse_button_input: Res<Input<MouseButton>>,
        ) {
            for i in raycast_source.into_iter() {
                if let PickingEvent::Selection(SelectionEvent::JustSelected(e)) = i {
                    /// in the event when a component is already being dragged, the actions should continue even if
                    /// the mouse is dragged over some other component, so only update if mouse button is not pressed.
                    if !mouse_button_input.pressed(MouseButton::Left) {
                        intersected.picked_ui_flag = true;
                    }
                }
                if let PickingEvent::Selection(SelectionEvent::JustDeselected(e)) = i {
                    intersected.picked_ui_flag = false;
                }
                if let PickingEvent::Hover(HoverEvent::JustEntered(e)) = i {
                    /// in the event when a component is already being dragged, the actions should continue even if
                    /// the mouse is dragged over some other component, so only update if mouse button is not pressed.
                    if !mouse_button_input.pressed(MouseButton::Left) {
                        intersected.picked_ui_flag = true;
                    }
                }
                 else if let PickingEvent::Clicked(e) = i {
                    intersected.picked_ui_flag = true;
                }

            }
        }
    };

    ($($event_writer_ident:ident, $event_query_ident:ident, $event_writer_type:ty),*) => {

        pub(crate) fn calculate_picks(
            $(
                mut $event_writer_ident: EventWriter<InteractionEvent<$event_writer_type>>,
                $event_query_ident: Query<(Entity, &PickableComponent), $event_writer_type>,
            ),*
            mut raycast_source: EventReader<PickingEvent>,
            mut intersected: ResMut<BevyPickingState>,
            cam: Res<ZoomableDraggableCamera>,
            mouse_button_input: Res<Input<MouseButton>>,
        ) {
            for i in raycast_source.into_iter() {

                if let PickingEvent::Selection(SelectionEvent::JustSelected(e)) = i {
                    if !mouse_button_input.pressed(MouseButton::Left) {
                        intersected.picked_ui_flag = true;
                    }
                }
                if let PickingEvent::Selection(SelectionEvent::JustDeselected(e)) = i {
                    intersected.picked_ui_flag = false;
                }
                if let PickingEvent::Hover(HoverEvent::JustEntered(e)) = i {
                    if !mouse_button_input.pressed(MouseButton::Left) {
                        intersected.picked_ui_flag = true;
                    }
                }
                 else if let PickingEvent::Clicked(e) = i {
                    intersected.picked_ui_flag = true;
                }

                $(
                    if <InteractionEvent<$event_writer_type> as MatchesPickingEvent>::matches(&i, $event_query_ident.get(get_entity(&i))) {
                        $event_writer_ident.send(InteractionEvent::RayCastInteraction {
                                event: crate::interactions::PickingEvent::from(i)
                        });
                    }
                )*
            }
        }
    };

}

ray_cast_events_system!(
    create_graph_menu_event_writer,
    create_graph_menu_query,
    PickableIxnFilter<GraphMenuPotential>
);