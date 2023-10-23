use bevy::prelude::*;
use crate::menu::UiComponent;
use crate::network::{ Node};

// pub(crate) fn update_components_selected<T>
// (
//     mut commands: Commands,
//     mut read_events: EventReader<PickingEvent>,
//     mut query: Query<(&Node), (With<T>, Without<UiComponent>)>,
//     mut ui_query: Query<(Entity, &UiComponent), With<UiComponent>>
// ) {
//     for picked in read_events.iter() {
//         if let PickingEvent::Clicked(entity) = picked {
//             info!("Picked!");
//         } else if let PickingEvent::Selection(SelectionEvent::JustSelected(entity)) = picked {
//             ui_query.iter_mut().for_each(|ui| {
//                 commands.get_entity(ui.0)
//                     .as_mut()
//                     .map(|e| {
//                         e.insert(Visibility::Hidden);
//                     });
//             });
//             info!("Selected!");
//         } else if let PickingEvent::Selection(SelectionEvent::JustDeselected(entity)) = picked {
//             ui_query.iter_mut().for_each(|ui| {
//                 commands.get_entity(ui.0)
//                     .as_mut()
//                     .map(|e| {
//                         e.insert(Visibility::Visible);
//                     });
//             });
//             info!("Selected!");
//         }
//     }
// }
//
