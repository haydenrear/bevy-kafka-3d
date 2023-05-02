use std::marker::PhantomData;
use bevy::ecs::query::{QueryEntityError, ReadOnlyWorldQuery, WorldQuery};
use bevy::prelude::{Commands, Component, Entity, EventReader, EventWriter, Interaction, Query, Res, ResMut, Resource};
use bevy::log::info;
use bevy::time::Time;
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_state::{StateChangeFactory, Update, UpdateStateInPlace};

pub fn write_events<
    RetrieveStateT,
    EventArgsT: EventArgs + 'static,
    EventDataT: EventData + 'static,
    ComponentT: Component + Send + Sync + 'static,
    SelfQuery: WorldQuery,
    SelfFilterQuery: ReadOnlyWorldQuery,
    ParentQuery: WorldQuery,
    ParentFilterQuery: ReadOnlyWorldQuery,
    ChildQuery: WorldQuery,
    ChildFilterQuery: ReadOnlyWorldQuery,
    InteractionFilterQuery: ReadOnlyWorldQuery,
>(
    mut commands: Commands,
    retrieve: ResMut<RetrieveStateT>,
    mut event_write: EventWriter<EventDescriptor<EventDataT, EventArgsT, ComponentT>>,
    self_query: Query<SelfQuery, SelfFilterQuery>,
    with_parent_query: Query<ParentQuery, ParentFilterQuery>,
    with_child_query: Query<ChildQuery, ChildFilterQuery>,
    interaction_query: Query<(Entity, &Interaction), InteractionFilterQuery>,
)
    where
        RetrieveStateT: RetrieveState<
            EventArgsT, EventDataT, ComponentT, SelfQuery,
            ParentQuery, ChildQuery, SelfFilterQuery,
            ParentFilterQuery, ChildFilterQuery
        >
{

    let _ = interaction_query
        .iter()
        .for_each(|(entity, interaction)| {
            if let Interaction::Clicked = interaction {
                RetrieveStateT::create_event(
                        &mut commands,
                        entity, &self_query,
                        &with_parent_query,
                        &with_child_query
                    )
                    .into_iter()
                    .for_each(|event| event_write.send(event));
            }
        });
}

pub trait InteractionEventReader<T, A, StateComponent, UpdateComponent, StateChangeFactoryI, StateUpdateI, QF: ReadOnlyWorldQuery = ()>
    where
        T: EventData + 'static,
        A: EventArgs + 'static,
        StateComponent: Component + Send + Sync + 'static,
        UpdateComponent: Component + Send + Sync + 'static,
        StateChangeFactoryI: StateChangeFactory<T, A, StateComponent, UpdateComponent, StateUpdateI>,
        StateUpdateI: UpdateStateInPlace<UpdateComponent>
{
    fn read_events(
        commands: Commands,
        update_state: PhantomData<StateChangeFactoryI>,
        read_events: EventReader<EventDescriptor<T, A, StateComponent>>,
        query: Query<(Entity, &mut UpdateComponent), QF>
    );
}

pub struct StateChangeEventReader;

impl <EventDataT, EventArgsT, StateComponent, UpdateComponent, StateChangeFactoryT, StateUpdateI, QF: ReadOnlyWorldQuery>
InteractionEventReader<EventDataT, EventArgsT, StateComponent, UpdateComponent, StateChangeFactoryT, StateUpdateI, QF>
for StateChangeEventReader
    where
        EventDataT: EventData + 'static,
        EventArgsT: EventArgs + 'static,
        StateComponent: Component + Send + Sync + 'static,
        UpdateComponent: Component + Send + Sync + 'static,
        StateChangeFactoryT: StateChangeFactory<EventDataT, EventArgsT, StateComponent, UpdateComponent, StateUpdateI>,
        StateUpdateI: UpdateStateInPlace<UpdateComponent>
{
    fn read_events(
        mut commands: Commands,
        update_state: PhantomData<StateChangeFactoryT>,
        mut read_events: EventReader<EventDescriptor<EventDataT, EventArgsT, StateComponent>>,
        mut query: Query<(Entity, &mut UpdateComponent), QF>
    ) {
        for event in read_events.iter() {
            StateChangeFactoryT::current_state(event)
                .iter()
                .for_each(|state| {
                    let _ = query.get_mut(state.entity)
                        // fetches a different component on the same entity for updating.
                        .map(|(entity, mut component)| {
                            // get the event to update
                            state.update_state(&mut commands, &mut component);
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
    EventArgsT,
    EventDataT,
    ComponentT,
    SelfQuery,
    ParentQuery,
    ChildQuery,
    SelfFilterQuery: ReadOnlyWorldQuery = (),
    ParentFilterQuery: ReadOnlyWorldQuery = (),
    ChildFilterQuery: ReadOnlyWorldQuery = (),
>: Resource
    where
        EventArgsT: EventArgs,
        EventDataT: EventData,
        ComponentT: Component + Send + Sync + 'static,
        SelfQuery: WorldQuery,
        ParentQuery: WorldQuery,
        ChildQuery: WorldQuery
{
    fn create_event(
        commands: &mut Commands,
        entity: Entity,
        self_query: &Query<SelfQuery, SelfFilterQuery>,
        with_parent_query: &Query<ParentQuery, ParentFilterQuery>,
        with_child_query: &Query<ChildQuery, ChildFilterQuery>
    ) ->  Vec<EventDescriptor<EventDataT, EventArgsT, ComponentT>>;
}
