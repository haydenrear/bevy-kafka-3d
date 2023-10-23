use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::ecs::query::{QueryEntityError, ReadOnlyWorldQuery, WorldQuery};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::{Commands, Component, CursorMoved, Entity, EventReader, EventWriter, Input, Interaction, MouseButton, Query, Res, ResMut, Resource};
use bevy::log::info;
use bevy::math::Vec2;
use crate::camera::raycast_select::BevyPickingState;
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_state::{ClickContext, Context, InsertComponent, InsertComponentChangeFactory, StateChangeFactory, Update, UpdateStateInPlace};
use crate::interactions::InteractionEvent;
use crate::menu::ui_menu_event::ui_state_change::{GlobalState, StateAdviser, StateChangeMachine, UpdateGlobalState};

pub trait EventsSystem<
    RetrieveStateT,
    EventArgsT: EventArgs + 'static + Debug,
    EventDataT: EventData + 'static,
    // component propagate events
    ComponentT: Component + Send + Sync + 'static + Debug,
    ComponentChangeT: Component + Send + Sync + 'static + Debug,
    Ctx: ClickContext<SelfFilterQuery, InteractionFilterQuery>,
    SelfQuery: WorldQuery,
    SelfFilterQuery: ReadOnlyWorldQuery + Send + Sync + 'static,
    PropagationQuery: WorldQuery,
    PropagationFilterQuery: ReadOnlyWorldQuery,
    InteractionFilterQuery: ReadOnlyWorldQuery + Send + Sync + 'static,
>
    where RetrieveStateT: RetrieveState<
        EventArgsT, EventDataT, Ctx, SelfQuery,
        PropagationQuery, ComponentT, ComponentChangeT, SelfFilterQuery,
        PropagationFilterQuery
        > + UpdateGlobalState<SelfFilterQuery, InteractionFilterQuery>  {
    fn click_write_events(
        mut commands: Commands,
        mut cursor_res: ResMut<RetrieveStateT>,
        mut context: ResMut<Ctx>,
        mut event_write: EventWriter<EventDescriptor<EventDataT, EventArgsT, ComponentChangeT>>,
        self_query: Query<SelfQuery, SelfFilterQuery>,
        propagation_query: Query<PropagationQuery, PropagationFilterQuery>,
        mut interaction_events: EventReader<InteractionEvent<InteractionFilterQuery>>,
        mouse_button_input: Res<Input<MouseButton>>,
        mut intersected: ResMut<BevyPickingState>,
        mut global_state: ResMut<GlobalState>
    )
    {

        let _ = interaction_events
            .iter()
            .for_each(|interaction| {
                if let InteractionEvent::UiComponentInteraction { event: Interaction::Pressed, entity} = interaction {
                    info!("Found click event with {:?}", entity);
                    context.clicked(*entity);
                    RetrieveStateT::update_hover_ui(&mut global_state, Some(*entity));
                    RetrieveStateT::update_click_hover_ui(&mut global_state, Some(*entity));
                    intersected.picked_ui_flag = true;
                    Self::propagate_events(
                        &mut commands,
                        &mut context,
                        &mut event_write,
                        &self_query,
                        &propagation_query,
                        entity
                    );
                } else if let InteractionEvent::UiComponentInteraction { event: Interaction::None, entity} = interaction {
                    context.un_clicked();
                    RetrieveStateT::update_click_hover_ui(&mut global_state, None);
                    RetrieveStateT::update_hover_ui(&mut global_state, None);
                    /// Cursor is set to zero if Changed because mouse button isn't pressed and not interacting with
                    /// anyone.
                    RetrieveStateT::update_cursor(&mut global_state, Vec2::ZERO);
                    if !mouse_button_input.pressed(MouseButton::Left) {
                        intersected.picked_ui_flag = false;
                    }
                } else if let InteractionEvent::UiComponentInteraction { event: Interaction::Hovered, entity } = interaction {
                    /// in the event when a component is already being dragged, the actions should continue even if
                    /// the mouse is dragged over some other component, so only update if mouse button is not pressed.
                    if !mouse_button_input.pressed(MouseButton::Left) {
                        intersected.picked_ui_flag = true;
                    }
                    RetrieveStateT::update_hover_ui(&mut global_state, Some(*entity));
                } else if let InteractionEvent::CursorEvent { event, .. } = interaction {
                    Self::update_cursor(&mut global_state, event);
                    global_state.click_hover_ui
                        .map(|entity| Self::propagate_events(
                            &mut commands,
                            &mut context,
                            &mut event_write,
                            &self_query,
                            &propagation_query,
                            &entity
                        ));
                } else if let InteractionEvent::ScrollWheelEvent { event } = interaction {
                    Self::update_mouse_wheel(&mut global_state, event);
                    global_state.hover_ui
                        .map(|entity| Self::propagate_events(
                            &mut commands,
                            &mut context,
                            &mut event_write,
                            &self_query,
                            &propagation_query,
                            &entity
                        ));
                } else if let InteractionEvent::RayCastInteraction { event} = interaction {

                }
            });
    }

    fn propagate_events(
        mut commands: &mut Commands,
        mut context: &mut ResMut<Ctx>,
        mut event_write: &mut EventWriter<EventDescriptor<EventDataT, EventArgsT, ComponentChangeT>>,
        self_query: &Query<SelfQuery, SelfFilterQuery>,
        propagation_query: &Query<PropagationQuery, PropagationFilterQuery>,
        entity: &Entity
    ) {
        let events = RetrieveStateT::create_event(
            &mut commands,
            *entity,
            &mut context,
            &self_query,
            &propagation_query
        );

        events.into_iter()
            .for_each(|(event)| event_write.send(event));
    }

    fn update_mouse_wheel(
        mut context: &mut ResMut<GlobalState>,
        event: &MouseWheel,
    ) {
        if RetrieveStateT::hover_ui(context) {
            RetrieveStateT::update_wheel(&mut context, Vec2::new(event.x, event.y), Some(event.unit));
        }
    }

    fn update_cursor(
        mut context: &mut ResMut<GlobalState>,
        event: &CursorMoved,
    ) {
        if RetrieveStateT::hover_ui(context) {
            RetrieveStateT::update_cursor(&mut context, event.position);
        }
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
        StateUpdateI: UpdateStateInPlace<UpdateComponent, Ctx>,
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
                        .map(|(entity, mut component)| {
                            /// The update component and the state component can be different. If the
                            /// state required to update the component spans multiple components, then this is
                            /// handled already and included in the NextStateChange.
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

pub trait InsertComponentInteractionEventReader<
    EventDataT, EventArgsT, NextEventComponentT, StateAdviserComponentT,
    StateChangeFactoryT, StateUpdateI,
    Ctx: Context + Debug + Clone,
    StateAdviserQueryFilterT: ReadOnlyWorldQuery
>
    where
        EventDataT: EventData + 'static + Debug,
        EventArgsT: EventArgs + 'static + Debug,
        StateAdviserComponentT: StateAdviser<NextEventComponentT> + Clone,
        NextEventComponentT: Component + Send + Sync + 'static + Debug + Clone,
        StateChangeFactoryT: InsertComponentChangeFactory<EventDataT, EventArgsT, NextEventComponentT, StateAdviserComponentT, Ctx, StateUpdateI>,
        StateUpdateI: InsertComponent<NextEventComponentT, StateAdviserComponentT, Ctx>,
{
    fn read_events(
        mut commands: Commands,
        mut ctx_resource: ResMut<Ctx>,
        update_state: PhantomData<StateChangeFactoryT>,
        mut read_events: EventReader<EventDescriptor<EventDataT, EventArgsT, NextEventComponentT>>,
        mut query: Query<(Entity, &StateAdviserComponentT), StateAdviserQueryFilterT>
    ) {
        for event in read_events.iter() {
            info!("Reading next component insert event event: {:?}", event);
            StateChangeFactoryT::current_state(event, &mut ctx_resource)
                .into_iter()
                .for_each(|state| {
                    info!("Reading next component insert state change: {:?}", state);
                    let _ = query.get_mut(state.adviser_component())
                        .map(|(entity, component)| {
                            /// The update component and the state component can be different. If the
                            /// state required to update the component spans multiple components, then this is
                            /// handled already and included in the NextStateChange.
                            state.insert_update_components(
                                &mut commands,
                                state.next_state(),
                                &mut ctx_resource,
                                state.entity_component(),
                                component
                            );
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
    Ctx,
    SelfQueryValues,
    PropagationQueryValues,
    ComponentStateT,
    ComponentChangeT = ComponentStateT,
    SelfFilter: ReadOnlyWorldQuery = (),
    PropagationFilter: ReadOnlyWorldQuery = (),
>: Resource
    where
        EventArgsT: EventArgs + Debug + 'static,
        EventDataT: EventData + 'static,
        ComponentStateT: Component + Send + Sync + 'static + Debug,
        ComponentChangeT: Component + Send + Sync + 'static + Debug,
        Ctx: Context,
        SelfQueryValues: WorldQuery,
        PropagationQueryValues: WorldQuery,
{
    /// Creates an event based on the current state of ComponentT as well as the current Ctx.
    /// If the next state generated depends on multiple components, then the context is used to store
    /// that information.
    fn create_event(
        commands: &mut Commands,
        entity: Entity,
        context: &mut ResMut<Ctx>,
        self_query: &Query<SelfQueryValues, SelfFilter>,
        propagation_query: &Query<PropagationQueryValues, PropagationFilter>,
    ) -> Vec<EventDescriptor<EventDataT, EventArgsT, ComponentChangeT>>;
}
