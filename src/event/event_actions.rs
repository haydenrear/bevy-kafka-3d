use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::ecs::query::{QueryEntityError, ReadOnlyWorldQuery, WorldQuery};
use bevy::prelude::{Commands, Component, Entity, EventReader, EventWriter, Interaction, Query, Res, ResMut, Resource};
use bevy::log::info;
use bevy::time::Time;
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_propagation::PropagateComponentEvent;
use crate::event::event_state::{Context, StateChangeFactory, Update, UpdateStateInPlace};
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

pub trait ClickWriteEvents <
    RetrieveStateT,
    EventArgsT: EventArgs + 'static,
    EventDataT: EventData + 'static,
    ComponentT: Component + Send + Sync + 'static,
    Ctx: Context,
    SelfQuery: WorldQuery,
    SelfFilterQuery: ReadOnlyWorldQuery,
    ParentQuery: WorldQuery,
    ParentFilterQuery: ReadOnlyWorldQuery,
    ChildQuery: WorldQuery,
    ChildFilterQuery: ReadOnlyWorldQuery,
    InteractionFilterQuery: ReadOnlyWorldQuery,
>
    where RetrieveStateT: RetrieveState<
            EventArgsT, EventDataT, ComponentT, Ctx, SelfQuery,
            ParentQuery, ChildQuery, SelfFilterQuery,
            ParentFilterQuery, ChildFilterQuery
        > {
    fn click_write_events(
        mut commands: Commands,
        retrieve: ResMut<RetrieveStateT>,
        mut context: ResMut<Ctx>,
        mut event_write: EventWriter<EventDescriptor<EventDataT, EventArgsT, ComponentT>>,
        self_query: Query<SelfQuery, SelfFilterQuery>,
        with_parent_query: Query<ParentQuery, ParentFilterQuery>,
        with_child_query: Query<ChildQuery, ChildFilterQuery>,
        interaction_query: Query<(Entity, &Interaction, &UiIdentifiableComponent), InteractionFilterQuery>,
        mut propagation_write: EventWriter<PropagateComponentEvent>
    )
    {
        let _ = interaction_query
            .iter()
            .for_each(|(entity, interaction, component)| {
                if let Interaction::Clicked = interaction {
                    info!("Had interaction with: {:?}", component);
                    let events = RetrieveStateT::create_event(
                        &mut commands,
                        entity, &mut context,
                        &self_query,
                        &with_parent_query,
                        &with_child_query
                    );

                    events.0.into_iter()
                        .for_each(|(event)| event_write.send(event));
                    events.1.into_iter()
                        .for_each(|(event)| propagation_write.send(event));
                }
            });

    }
}

pub trait InteractionEventReader<
    EventDataT, EventArgsT, StateComponent,
    UpdateComponent, StateChangeFactoryT, StateUpdateI,
    Ctx: Context + Debug,
    QF: ReadOnlyWorldQuery
>
    where
        EventDataT: EventData + 'static + Debug,
        EventArgsT: EventArgs + 'static + Debug,
        StateComponent: Component + Send + Sync + 'static + Debug,
        UpdateComponent: Component + Send + Sync + 'static + Debug,
        StateChangeFactoryT: StateChangeFactory<EventDataT, EventArgsT, StateComponent, UpdateComponent, Ctx, StateUpdateI>,
        StateUpdateI: UpdateStateInPlace<UpdateComponent, Ctx>
{
    fn read_events(
        mut commands: Commands,
        mut ctx_resource: ResMut<Ctx>,
        update_state: PhantomData<StateChangeFactoryT>,
        mut read_events: EventReader<EventDescriptor<EventDataT, EventArgsT, StateComponent>>,
        mut query: Query<(Entity, &mut UpdateComponent), QF>
    ) {
        for event in read_events.iter() {
            info!("Reading next event: {:?}", event);
            StateChangeFactoryT::current_state(event, &mut ctx_resource)
                .iter()
                .for_each(|state| {
                    info!("Reading next state change: {:?}", state);
                    let _ = query.get_mut(state.entity)
                        // fetches a different component on the same entity for updating.
                        .map(|(entity, mut component)| {
                            // get the event to update
                            state.update_state(&mut commands, &mut component, &mut ctx_resource);
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
    Ctx,
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
        Ctx: Context,
        SelfQuery: WorldQuery,
        ParentQuery: WorldQuery,
        ChildQuery: WorldQuery
{
    fn create_event(
        commands: &mut Commands,
        entity: Entity,
        context: &mut ResMut<Ctx>,
        self_query: &Query<SelfQuery, SelfFilterQuery>,
        with_parent_query: &Query<ParentQuery, ParentFilterQuery>,
        with_child_query: &Query<ChildQuery, ChildFilterQuery>
    ) ->  (Vec<EventDescriptor<EventDataT, EventArgsT, ComponentT>>, Vec<PropagateComponentEvent>);
}
