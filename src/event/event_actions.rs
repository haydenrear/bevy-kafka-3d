use bevy::ecs::query::{QueryEntityError, ReadOnlyWorldQuery, WorldQuery};
use bevy::prelude::{Commands, Component, Entity, EventReader, EventWriter, Interaction, Query, Res, ResMut, Resource};
use bevy::log::info;
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_state::{StateChangeFactory, UpdateStateInPlace};

pub fn write_events<
    State,
    ClickState,
    EArgs: EventArgs + 'static,
    EvD: EventData + 'static,
    C: Component + Send + Sync + 'static,
    SelfQuery: WorldQuery,
    ParentQuery: WorldQuery,
    ChildQuery: WorldQuery,
    SelfFilterQuery: ReadOnlyWorldQuery,
    ParentFilterQuery: ReadOnlyWorldQuery,
    ChildFilterQuery: ReadOnlyWorldQuery,
    InteractionFilterQuery: ReadOnlyWorldQuery,
>(
    mut commands: Commands,
    retrieve: ResMut<ClickState>,
    mut event_write: EventWriter<EventDescriptor<EvD, EArgs, C>>,
    self_query: Query<SelfQuery, SelfFilterQuery>,
    with_parent_query: Query<ParentQuery, ParentFilterQuery>,
    with_child_query: Query<ChildQuery, ChildFilterQuery>,
    interaction_query: Query<(Entity, &Interaction), InteractionFilterQuery>,
)
    where
        ClickState: RetrieveState<
            EArgs, EvD, C, SelfQuery, ParentQuery, ChildQuery, SelfFilterQuery,
            ParentFilterQuery, ChildFilterQuery
        >,
        State: RetrieveState<
            EArgs,
            EvD,
            C,
            SelfQuery,
            ParentQuery,
            ChildQuery,
            SelfFilterQuery,
            ParentFilterQuery,
            ChildFilterQuery
        >
{

    let _ = interaction_query
        .iter()
        .for_each(|(entity, interaction)| {
            if let Interaction::Clicked = interaction {
                ClickState::retrieve_state(
                        &mut commands,
                        entity, &self_query,
                        &with_parent_query,
                        &with_child_query
                    )
                    .map(|event| event_write.send(event));
            }
        });
}

pub trait EventReaderT<T, A, C, StateChangeFactoryI, StateUpdateI, QF: ReadOnlyWorldQuery = ()>
    where
        T: EventData + 'static,
        A: EventArgs + 'static,
        C: Component + Send + Sync + 'static,
        StateChangeFactoryI: StateChangeFactory<T, A, C, StateUpdateI>,
        StateUpdateI: UpdateStateInPlace<C>
{
    fn read_events(
        commands: Commands,
        update_stae: Res<StateChangeFactoryI>,
        read_events: EventReader<EventDescriptor<T, A, C>>,
        query: Query<(Entity, &mut C), QF>
    );
}

pub struct StateChangeEventReader;

impl <T, A, C, StateChangeFactoryI, StateUpdateI, QF: ReadOnlyWorldQuery>
EventReaderT<T, A, C, StateChangeFactoryI, StateUpdateI, QF>
for StateChangeEventReader
    where
        T: EventData + 'static,
        A: EventArgs + 'static,
        C: Component + Send + Sync + 'static,
        StateChangeFactoryI: StateChangeFactory<T, A, C, StateUpdateI>,
        StateUpdateI: UpdateStateInPlace<C>
{
    fn read_events(
       mut commands: Commands,
       update_state: Res<StateChangeFactoryI>,
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


/// Fetch the information about the event, such as the child and parent values, to be included
/// in the event.
pub trait RetrieveState<
    A,
    D,
    C,
    SelfQuery,
    ParentQuery,
    ChildQuery,
    SelfFilterQuery: ReadOnlyWorldQuery = (),
    ParentFilterQuery: ReadOnlyWorldQuery = (),
    ChildFilterQuery: ReadOnlyWorldQuery = (),
>: Resource
    where
        A: EventArgs,
        D: EventData,
        C: Component + Send + Sync + 'static,
        SelfQuery: WorldQuery,
        ParentQuery: WorldQuery,
        ChildQuery: WorldQuery
{
    fn retrieve_state(
        commands: &mut Commands,
        entity: Entity,
        self_query: &Query<SelfQuery, SelfFilterQuery>,
        with_parent_query: &Query<ParentQuery, ParentFilterQuery>,
        with_child_query: &Query<ChildQuery, ChildFilterQuery>
    ) ->  Option<EventDescriptor<D, A, C>>;
}
