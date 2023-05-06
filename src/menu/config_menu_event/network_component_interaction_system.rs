use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, SelectionEvent};
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiComponent;
use crate::network::{HasMetrics, Node};

/// When Layer, Network, Node components selected, remove menu component and show new menu
pub(crate) fn update_components_selected<T: HasMetrics<T> + Component, U: Component>
(
    mut commands: Commands,
    mut read_events: EventReader<PickingEvent>,
    mut query: Query<(&Node), (With<T>, Without<UiComponent>)>,
    mut ui_query: Query<(Entity, &UiComponent), With<UiComponent>>
) {
    for picked in read_events.iter() {
        if let PickingEvent::Clicked(entity) = picked {
            info!("Picked!");
        } else if let PickingEvent::Selection(SelectionEvent::JustSelected(entity)) = picked {
            ui_query.iter_mut().for_each(|ui| {
                commands.get_entity(ui.0)
                    .as_mut()
                    .map(|e| {
                        e.insert(Visibility::Hidden);
                    });
            });
            info!("Selected!");
        } else if let PickingEvent::Selection(SelectionEvent::JustDeselected(entity)) = picked {
            ui_query.iter_mut().for_each(|ui| {
                commands.get_entity(ui.0)
                    .as_mut()
                    .map(|e| {
                        e.insert(Visibility::Visible);
                    });
            });
            info!("Selected!");
        }
    }
}

