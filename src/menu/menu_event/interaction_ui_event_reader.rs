use bevy::prelude::{BackgroundColor, Commands, Component, Entity, EventReader, ParamSet, Query, Res, Style, With};
use bevy::log::info;
use bevy::ecs::query::{QueryEntityError, ReadOnlyWorldQuery};
use crate::menu::menu_event::{ClickStateChangeState, EventArgs, EventData, EventDescriptor, NextUiState, StateChangeActionComponentStateFactory, StateChangeActionType, UiComponent, StateChangeFactory, UiEventArgs, UpdateStateInPlace};
use crate::menu::menu_event::interaction_ui_event_writer::StateChangeActionTypeStateRetriever;
use crate::visualization::UiIdentifiableComponent;

pub trait EventReaderT<T, A, C, StateChangeFactoryI, StateUpdateI, QF: ReadOnlyWorldQuery = ()>
    where
        T: EventData + 'static,
        A: EventArgs + 'static,
        C: Component + Send + Sync + 'static,
        StateChangeFactoryI: StateChangeFactory<T, A, C, StateUpdateI>,
        StateUpdateI: UpdateStateInPlace<C>
{
    fn read_event(
        update_stae: Res<StateChangeFactoryI>,
        read_events: EventReader<EventDescriptor<T, A, C>>,
        query: Query<(Entity, &mut C), QF>
    );
}

pub struct EventReaderImpl;

impl <T, A, C, StateChangeFactoryI, StateUpdateI, QF: ReadOnlyWorldQuery>
EventReaderT<T, A, C, StateChangeFactoryI, StateUpdateI, QF>
for EventReaderImpl
    where
        T: EventData + 'static,
        A: EventArgs + 'static,
        C: Component + Send + Sync + 'static,
        StateChangeFactoryI: StateChangeFactory<T, A, C, StateUpdateI>,
        StateUpdateI: UpdateStateInPlace<C>
{
    fn read_event(update_state: Res<StateChangeFactoryI>,
                  mut read_events: EventReader<EventDescriptor<T, A, C>>,
                  mut query: Query<(Entity, &mut C), QF>
    ) {
        for event in read_events.iter() {
            StateChangeFactoryI::current_state(event)
                .iter()
                .for_each(|state| {
                    let _ = query.get_mut(state.entity)
                        .map(|(entity, mut component)| {
                            state.update_state(&mut component);
                        })
                        .or_else(|f| {
                            info!("Failed to fetch query: {:?}.", f);
                            Ok::<(), QueryEntityError>(())
                        });
                });
        }
    }
}

pub fn read_menu_ui_event(
    mut read_events: EventReader<EventDescriptor<StateChangeActionType, UiEventArgs, Style>>,
    mut query: Query<
        (Entity, &mut Style),
        (With<UiComponent>)
    >
) {
    for mut event in read_events.iter() {
        StateChangeActionComponentStateFactory::current_state(event)
            .iter()
            .for_each(|state_change| {
                match &state_change.next_state {
                    NextUiState::ReplaceSize(size) => {
                        let _ = query
                            .get_mut(state_change.entity.clone())
                            .map(|(_, mut style)| {
                                size.update_state(&mut style.size);
                            })
                            .or_else(|_| {
                                info!("Failed to update color.");
                                Ok::<(), QueryEntityError>(())
                            });
                    }
                    NextUiState::ReplaceDisplay(display) => {
                        let _ = query
                            .get_mut(state_change.entity.clone())
                            .map(|(_, mut style)| {
                                display.update_state(&mut style.display);
                            })
                            .or_else(|_| {
                                info!("Failed to update color.");
                                Ok::<(), QueryEntityError>(())
                            });
                    }
                }
            });
    }
}
