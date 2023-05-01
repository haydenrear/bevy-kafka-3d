use bevy::prelude::{BackgroundColor, Commands, Entity, EventReader, ParamSet, Query, Style, With};
use bevy::log::info;
use bevy::ecs::query::QueryEntityError;
use crate::menu::menu_event::{ClickStateChangeState, UiComponent, UiEvent};
use crate::visualization::UiIdentifiableComponent;

pub fn read_menu_ui_event(
    mut commands: Commands,
    mut read_events: EventReader<UiEvent>,
    mut query: ParamSet<(
        Query<(Entity, &UiComponent, &mut Style, &UiIdentifiableComponent), (With<UiComponent>)>,
        Query<(Entity, &UiComponent, &mut BackgroundColor, &UiIdentifiableComponent), (With<UiComponent>)>
    )>,
) {
    for event in read_events.iter() {
        if let UiEvent::Event(ClickStateChangeState::ChangeColor { current_display, update_display }) = event {
            update_display.iter().for_each(|(entity, color)| {
                let _ = query.p1()
                    .get_mut(entity.clone())
                    .map(|(_, _, mut color_update, _)| {
                        color_update.0 = color.clone();
                    })
                    .or_else(|_| {
                        info!("Failed to update color.");
                        Ok::<(), QueryEntityError>(())
                    });
            });
        } else if let UiEvent::Event(ClickStateChangeState::ChangeSize { current_display, update_display }) = event {
            update_display.iter().for_each(|(entity, size)| {
                let _ = query.p0()
                    .get_mut(entity.clone())
                    .map(|(_, _, mut style, _)| {
                        style.size = size.clone();
                    })
                    .or_else(|_| {
                        info!("Failed to update color.");
                        Ok::<(), QueryEntityError>(())
                    });
            });
        } else if let UiEvent::Event(ClickStateChangeState::ChangeDisplay { current_display, update_display }) = event {
            update_display.iter().for_each(|(entity, display)| {
                let _ = query.p0()
                    .get_mut(entity.clone())
                    .map(|(_, _, mut style, _)| {
                        style.display = display.clone();
                    })
                    .or_else(|_| {
                        info!("Failed to update color.");
                        Ok::<(), QueryEntityError>(())
                    });
            });
        }
    }
}
