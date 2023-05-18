use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::ecs::query::{QueryEntityError, ReadOnlyWorldQuery, WorldQuery};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::{Commands, Component, CursorMoved, Entity, EventReader, EventWriter, Input, Interaction, MouseButton, Query, Res, ResMut, Resource};
use bevy::log::info;
use bevy::math::Vec2;
use bevy::time::Time;
use crate::camera::raycast_select::BevyPickingState;
use crate::cursor_adapter::CursorResource;
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_propagation::PropagateComponentEvent;
use crate::event::event_state::{ClickContext, Context, StateChangeFactory, Update, UpdateStateInPlace};
use crate::menu::ui_menu_event::interaction_ui_event_writer::{GlobalState, UpdateGlobalState};
use crate::menu::ui_menu_event::ui_state_change::StateChangeMachine;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

pub trait ClickWriteEvents <
    RetrieveStateT,
    EventArgsT: EventArgs + 'static + Debug,
    EventDataT: EventData + 'static,
    ComponentT: Component + Send + Sync + 'static + Debug,
    Ctx: ClickContext<SelfFilterQuery, InteractionFilterQuery>,
    SelfQuery: WorldQuery,
    SelfFilterQuery: ReadOnlyWorldQuery + Send + Sync + 'static,
    PropagationQuery: WorldQuery,
    PropagationFilterQuery: ReadOnlyWorldQuery,
    InteractionFilterQuery: ReadOnlyWorldQuery + Send + Sync + 'static,
>
    where RetrieveStateT: RetrieveState<
        EventArgsT, EventDataT, ComponentT, Ctx, SelfQuery,
        PropagationQuery, SelfFilterQuery,
        PropagationFilterQuery
        > + UpdateGlobalState<SelfFilterQuery, InteractionFilterQuery>  {
    fn click_write_events(
        mut commands: Commands,
        mut cursor_res: ResMut<RetrieveStateT>,
        mut context: ResMut<Ctx>,
        mut event_write: EventWriter<EventDescriptor<EventDataT, EventArgsT, ComponentT>>,
        self_query: Query<SelfQuery, SelfFilterQuery>,
        with_parent_query: Query<PropagationQuery, PropagationFilterQuery>,
        interaction_query: Query<(Entity, &Interaction, &UiIdentifiableComponent), InteractionFilterQuery>,
        mut propagation_write: EventWriter<PropagateComponentEvent>,
        mut cursor_events: EventReader<CursorMoved>,
        mut wheel: EventReader<MouseWheel>,
        mut intersected: ResMut<BevyPickingState>,
        mouse_button_input: Res<Input<MouseButton>>,
        mut global_state: ResMut<GlobalState>
    )
    {
        let _ = interaction_query
            .iter()
            .for_each(|(entity, interaction, _)| {
                if let Interaction::Clicked = interaction {
                    context.clicked();
                    RetrieveStateT::update_click_hover_ui(&mut global_state, true);
                    intersected.picked_ui_flag = true;
                    Self::update_cursor_res(&mut global_state, &mut cursor_events,  &mut wheel);

                    info!("Click interaction with: {:?}", &entity);
                    let events = RetrieveStateT::create_event(
                        &mut commands,
                        entity,
                        &mut context,
                        &self_query,
                        &with_parent_query
                    );

                    events.0.into_iter()
                        .for_each(|(event)| event_write.send(event));
                    events.1.into_iter()
                        .for_each(|(event)| propagation_write.send(event));

                } else if let Interaction::None = interaction {
                    context.un_clicked();
                    RetrieveStateT::update_click_hover_ui(&mut global_state, false);
                    RetrieveStateT::update_cursor(&mut global_state, Vec2::ZERO);
                    if !mouse_button_input.pressed(MouseButton::Left) {
                        intersected.picked_ui_flag = false;
                    }
                } else if let Interaction::Hovered = interaction {
                    if !mouse_button_input.pressed(MouseButton::Left) {
                        intersected.picked_ui_flag = true;
                    }
                    RetrieveStateT::update_hover_ui(&mut global_state, false);
                    Self::update_cursor_res(&mut global_state, &mut cursor_events, &mut wheel);
                }
            });
    }

    fn update_cursor_res(
        mut context: &mut ResMut<GlobalState>,
        cursor_events: &mut EventReader<CursorMoved>,
        mouse_wheel: &mut EventReader<MouseWheel>,
    ) {
        info!("did");
        if RetrieveStateT::click_hover_ui(context) {
            for event in cursor_events.iter() {
                RetrieveStateT::update_cursor(&mut context, event.position);
            }
            for event in mouse_wheel.iter() {
                RetrieveStateT::update_wheel(&mut context, Vec2::new(event.x, event.y), Some(event.unit));
            }
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
    PropagationQuery,
    SelfFilterQuery: ReadOnlyWorldQuery = (),
    PropagationFilterQuery: ReadOnlyWorldQuery = (),
>: Resource
    where
        EventArgsT: EventArgs + Debug + 'static,
        EventDataT: EventData + 'static,
        ComponentT: Component + Send + Sync + 'static + Debug,
        Ctx: Context,
        SelfQuery: WorldQuery,
        PropagationQuery: WorldQuery,
{
    fn create_event(
        commands: &mut Commands,
        entity: Entity,
        context: &mut ResMut<Ctx>,
        self_query: &Query<SelfQuery, SelfFilterQuery>,
        propagation_query: &Query<PropagationQuery, PropagationFilterQuery>,
    ) ->  (Vec<EventDescriptor<EventDataT, EventArgsT, ComponentT>>, Vec<PropagateComponentEvent>);
}
