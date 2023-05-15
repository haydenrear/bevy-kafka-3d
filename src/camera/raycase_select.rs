use bevy::log::info;
use bevy::prelude::{Entity, EventReader, Query, Res, ResMut, Resource};
use bevy_mod_picking::{PickingEvent, PickingRaycastSet, RaycastSource, SelectionEvent};
use crate::camera::ZoomableDraggableCamera;

#[derive(Resource)]
pub struct BevyPickingState {
    is_picked: bool
}

impl Default for BevyPickingState {
    fn default() -> Self {
       Self {
           is_picked: false
       }
    }
}

pub(crate) fn calculate_pick(
    mut raycast_source: EventReader<PickingEvent>,
    intersected: ResMut<BevyPickingState>,
    cam: Res<ZoomableDraggableCamera>
) {
    for i in raycast_source.iter() {
        // if camera is already dragging, then ignore hover - if camera is not dragging and hovering and
        // click and drag, then ignore camera movement.
        info!("selected: {:?}", i);
    }
}