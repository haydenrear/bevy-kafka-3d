use bevy::input::Input;
use bevy::log::info;
use bevy::prelude::{Entity, EventReader, MouseButton, Query, Res, ResMut, Resource};
use bevy_mod_picking::{HoverEvent, PickingEvent, PickingRaycastSet, RaycastSource, SelectionEvent};
use crate::camera::ZoomableDraggableCamera;

/// # Purpose
/// Tells the UI system that there is nothing picked in the bevy picking system. Important because
/// otherwise the camera will move as the user is dragging various UI menu items.
#[derive(Resource)]
pub struct BevyPickingState {
    pub(crate) picked_ui_flag: bool
}

impl Default for BevyPickingState {
    fn default() -> Self {
       Self {
           picked_ui_flag: false
       }
    }
}

