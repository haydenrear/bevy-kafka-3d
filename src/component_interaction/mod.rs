use bevy::prelude::*;
use bevy::sprite::Sprite;
use bevy_mod_picking::PickingEvent;
use crate::camera::ZoomableDraggableCamera;
use crate::network::Node;

#[derive(Default, Component)]
pub struct Highlightable {
    is_highlighted: bool
}


pub(crate) fn highlight_nodes(
    mut events: EventReader<PickingEvent>,
) {
    for query in events.iter() {
        match query {
            PickingEvent::Selection(_) => {
                info!("yes");
            }
            PickingEvent::Hover(_) => {
                info!("yes");
            }
            PickingEvent::Clicked(_) => {
                info!("yes");
            }
        }
    }
}