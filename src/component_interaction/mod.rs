use bevy::prelude::*;
use bevy::prelude::GamepadButtonType::Select;
use bevy::sprite::Sprite;
use bevy_mod_picking::{Hover, PickableMesh, PickingEvent, Selection, SelectionEvent};
use bevy_mod_picking::highlight::Highlight;
use bevy_mod_picking::SelectionEvent::JustSelected;
use crate::camera::ZoomableDraggableCamera;
use crate::network::Node;

#[derive(Default, Component)]
pub struct Highlightable {
    is_highlighted: bool
}

pub(crate) fn pick_node(
    mut picked: EventReader<PickingEvent>,
    selected_node: Query<&Node>
) {
    for pick in picked.iter() {
        if let PickingEvent::Selection(JustSelected(entity)) = *pick {
            let found_node = selected_node.get(entity).unwrap();
            info!("selected a node");
        }
    }
}
