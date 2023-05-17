use std::marker::PhantomData;
use bevy::ecs::query::{QueryEntityError, ReadOnlyWorldQuery};
use bevy::log::info;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::CursorMoved;
use crate::camera::raycast_select::BevyPickingState;
use crate::interactions::InteractionEvent;
use bevy::input::Input;
use bevy::input::mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel};
use bevy::prelude::{Entity, EventReader, MouseButton, Query, Res, ResMut, Resource};
use bevy_mod_picking::{HoverEvent, PickingEvent, PickingRaycastSet, RaycastSource, SelectionEvent};
use crate::camera::ZoomableDraggableCamera;


#[derive(Resource, Default)]
pub struct CursorResource {
    pub(crate) cursor_pos: Vec2,
    pub(crate) cursor_delta: Vec2,
    pub(crate) click_hover_ui: bool,
    pub(crate) hover_ui: bool,
    pub(crate) scroll_wheel_delta: Vec2,
    pub(crate) wheel_units: Option<MouseScrollUnit>
}

/// Will be used to adapt all events into a single InteractionEvent type, which is generic over
/// the query which is used, so that events can be filtered for the different Ui systems. Ultimately,
/// the event system needs to be used for both UI events and other event types, updating the state
/// of 3d elements and UI elements.
pub(crate) fn click_write_events<IXN: ReadOnlyWorldQuery + Send + Sync + 'static>(
    mut event_writer: EventWriter<InteractionEvent<IXN>>,
    interaction_query: Query<(Entity, &Interaction), IXN>,
)
{
    let _ = interaction_query
        .into_iter()
        .for_each(|(_, interaction)| {
            event_writer.send(InteractionEvent::BevyUiInteraction {
                event: *interaction
            })
        });
}


pub trait MatchesPickingEvent {
    fn matches(picking_event: &PickingEvent, raycast_actionable: Result<(Entity, &RaycastActionable), QueryEntityError>) -> bool ;
}

impl MatchesPickingEvent for InteractionEvent<()> {
    fn matches(picking_event: &PickingEvent, raycast_actionable: Result<(Entity, &RaycastActionable), QueryEntityError>) -> bool {
        raycast_actionable
            .map(|(entity, r)| {
                r.is_ui_interactable
            })
            .or::<QueryEntityError>(Ok(false))
            .unwrap()
    }
}

/// When an event happens with the raycast, maybe the event will want to be included so that some
/// action can be taken. This allows interaction between the 3d and the UI event system. When the
/// nodes are selected, a menu needs to pop up.
#[derive(Component)]
pub struct RaycastActionable {
    is_ui_interactable: bool,
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


macro_rules! ray_cast_system {
    () => {
        pub(crate) fn calculate_picks(
            mut raycast_source: EventReader<PickingEvent>,
            raycast_actionable: Query<(Entity, &RaycastActionable), (With<RaycastActionable>)>,
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

            }
        }
    };

    ($($event_writer_ident:ident: $event_writer_type:ty),*) => {

        pub(crate) fn calculate_picks(
            $($event_writer_ident: &mut EventWriter<InteractionEvent<$event_writer_type>>),*,
            mut raycast_source: EventReader<PickingEvent>,
            raycast_actionable: Query<(Entity, &RaycastActionable), (With<RaycastActionable>)>,
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
                    if <InteractionEvent<$event_writer_type> as MatchesPickingEvent>::matches(&i, raycast_actionable.get_single(get_entity(&i))) {
                        $event_writer_ident.send(InteractionEvent::RaycastInteraction {
                                event: crate::interactions::PickingEvent::from(i)
                        });
                    }
                )*
            }
        }
    };

}

ray_cast_system!();