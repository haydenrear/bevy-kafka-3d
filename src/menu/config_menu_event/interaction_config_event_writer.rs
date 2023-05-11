use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use crate::event::event_actions::{ClickWriteEvents, RetrieveState};
use crate::event::event_descriptor::EventDescriptor;
use crate::event::event_propagation::PropagateComponentEvent;
use crate::event::event_state::Context;
use crate::menu::{MetricsConfigurationOption, ConfigurationOptionComponent, ConfigurationOptionEnum, DataType, MenuType};
use crate::menu::config_menu_event::config_event::{ConfigurationOptionChange, ConfigurationOptionEventArgs};
use crate::menu::menu_resource::MENU;
use crate::network::{Network, Node};
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

#[derive(Default, Resource, Debug)]
pub struct ConfigOptionActionStateRetriever<T: Component> {
    phantom: PhantomData<T>,
}

#[derive(Default, Resource, Debug)]
pub struct ConfigOptionContext {
    pub(crate) graph_parent_entity: Option<Entity>,
    pub(crate) network_entity: Option<Entity>,
}

impl Context for ConfigOptionContext {}

impl<T: Component + Debug + Default + Clone> ClickWriteEvents<
    ConfigOptionActionStateRetriever<T>, ConfigurationOptionEventArgs<T>,
    DataType, MetricsConfigurationOption<T>, ConfigOptionContext,
    // self query
    (Entity, &MetricsConfigurationOption<T>),
    // self filter
    (With<MetricsConfigurationOption<T>>),
    // parent query
    (Entity, &Parent, &MetricsConfigurationOption<T>),
    // parent filter
    (With<Parent>, With<MetricsConfigurationOption<T>>),
    // child query
    (Entity, &Children, &MetricsConfigurationOption<T>),
    // child filter
    (With<Children>, With<MetricsConfigurationOption<T>>),
    // interaction filter
    (With<MetricsConfigurationOption<T>>, With<Button>, Changed<Interaction>)
> for ConfigOptionActionStateRetriever<T> {}

impl<T: Component + Send + Sync + Default + Clone + Debug + 'static> RetrieveState<
    ConfigurationOptionEventArgs<T>,
    DataType,
    MetricsConfigurationOption<T>,
    ConfigOptionContext,
    (Entity, &MetricsConfigurationOption<T>),
    (Entity, &Parent, &MetricsConfigurationOption<T>),
    (Entity, &Children, &MetricsConfigurationOption<T>),
    (With<MetricsConfigurationOption<T>>),
    (With<Parent>, With<MetricsConfigurationOption<T>>),
    (With<Children>, With<MetricsConfigurationOption<T>>),
>
for ConfigOptionActionStateRetriever<T>
{
    fn create_event(
        commands: &mut Commands,
        entity: Entity,
        mut context: &mut ResMut<ConfigOptionContext>,
        self_query: &Query<
            (Entity, &MetricsConfigurationOption<T>),
            (With<MetricsConfigurationOption<T>>)
        >,
        with_parent_query: &Query<
            (Entity, &Parent, &MetricsConfigurationOption<T>),
            (With<Parent>, With<MetricsConfigurationOption<T>>)
        >,
        with_child_query: &Query<
            (Entity, &Children, &MetricsConfigurationOption<T>),
            (With<Children>, With<MetricsConfigurationOption<T>>)
        >,
    ) -> (
        Vec<EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>>,
        Vec<PropagateComponentEvent>
    )
    {
        let mut event_descriptors = vec![];
        let mut propagate_events = vec![];
        info!("Here is ctx: {:?}", context);
        Self::set_menu_events(entity, context, self_query, &mut event_descriptors, &mut propagate_events);
        (event_descriptors, propagate_events)
    }
}

impl<T> ConfigOptionActionStateRetriever<T>
where T: Component + Send + Sync + Default + Clone + Debug + 'static
{
    fn set_menu_events(entity: Entity, mut context: &mut ResMut<ConfigOptionContext>, self_query: &Query<(Entity, &MetricsConfigurationOption<T>), With<MetricsConfigurationOption<T>>>, mut event_descriptors: &mut Vec<EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>>, mut propagate_events: &mut Vec<PropagateComponentEvent>) {
        let _ = self_query.get(entity)
            .map(|(entity, config)| {
                info!("Here is config for creating event: {:?}", config);
                match config {
                    MetricsConfigurationOption::GraphMenu(_, data_type, key, MenuType::Graph) => {
                        if let DataType::Selected = data_type {
                            let config_menu = MetricsConfigurationOption::GraphMenu(
                                PhantomData::<T>::default(),
                                DataType::Deselected,
                                key,
                                MenuType::Graph,
                            );
                            create_add_events(
                                &mut event_descriptors,
                                &mut propagate_events,
                                entity,
                                config_menu,
                                DataType::Deselected,
                                Visibility::Hidden,
                                context.graph_parent_entity,
                            );
                        } else if let DataType::Deselected = data_type {
                            let config_menu = MetricsConfigurationOption::GraphMenu(
                                PhantomData::<T>::default(),
                                DataType::Selected,
                                key,
                                MenuType::Graph,
                            );
                            create_add_events(&mut event_descriptors,
                                              &mut propagate_events,
                                              entity,
                                              config_menu,
                                              DataType::Selected,
                                              Visibility::Visible,
                                              context.graph_parent_entity,
                            );
                        }
                    }
                    MetricsConfigurationOption::NetworkMenu(_, data_type, key, MenuType::Network) => {
                        if let DataType::Selected = data_type {
                            let config_menu = MetricsConfigurationOption::NetworkMenu(
                                PhantomData::<T>::default(),
                                DataType::Deselected,
                                key,
                                MenuType::Network,
                            );
                            create_add_events(&mut event_descriptors,
                                              &mut propagate_events,
                                              entity,
                                              config_menu,
                                              DataType::Deselected,
                                              Visibility::Hidden,
                                              context.network_entity,
                            );
                        } else if let DataType::Deselected = data_type {
                            let config_menu = MetricsConfigurationOption::NetworkMenu(
                                PhantomData::<T>::default(),
                                DataType::Selected,
                                key,
                                MenuType::Network,
                            );
                            create_add_events(&mut event_descriptors,
                                              &mut propagate_events,
                                              entity,
                                              config_menu,
                                              DataType::Selected,
                                              Visibility::Visible,
                                              context.network_entity,
                            );
                        }
                    }
                    _ => {}
                }
            })
            .or_else(|e| {
                error!("Error with entity: {:?}", e);
                Err(e)
            });
    }
}

fn create_add_events<T>(
    mut event_descriptors: &mut Vec<EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>>,
    mut propagate_events: &mut Vec<PropagateComponentEvent>,
    entity: Entity,
    config: MetricsConfigurationOption<T>,
    data_type: DataType,
    visible: Visibility,
    other_entity: Option<Entity>,
)
    where
        T: Component + Send + Sync + Default + Clone + Debug + 'static
{
    let event_descriptor = create_graph_menu_event(entity, data_type, config);
    let event = create_event_tuple(
        event_descriptor,
        &|entity| Some(PropagateComponentEvent::ChangeVisible(entity, visible)),
        other_entity,
    );
    add_to_events(&mut event_descriptors, &mut propagate_events, event);
}

fn add_to_events<T>(
    mut event_descriptors:
    &mut Vec<EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>>,
    mut propagate_events: &mut Vec<PropagateComponentEvent>,
    event: Vec<(EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>, Vec<PropagateComponentEvent>)>,
)
    where
        T: Component + Send + Sync + Default + Clone + Debug + 'static
{
    event.into_iter().for_each(|(event, prop)| {
        info!("Adding event: {:?} and prop events: {:?}", &event, &prop);
        event_descriptors.push(event);
        prop.into_iter().for_each(|prop| propagate_events.push(prop));
    });
}

fn create_event_tuple<T>(
    event_descriptor: EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>,
    option: &dyn Fn(Entity) -> Option<PropagateComponentEvent>,
    entity: Option<Entity>,
) -> Vec<(EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>, Vec<PropagateComponentEvent>)>
    where
        T: Component + Send + Sync + Default + Clone + Debug + 'static
{
    let mut event_tuple = (event_descriptor, vec![]);
    if entity.is_some() {
        event_tuple.1 = option(entity.unwrap()).map(|p| vec![p]).or(Some(vec![])).unwrap();
    }
    vec![event_tuple]
}

fn create_graph_menu_event<T>(
    entity: Entity,
    data_type: DataType,
    config: MetricsConfigurationOption<T>) -> EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>
    where
        T: Component + Send + Sync + Default + Clone + Debug + 'static
{
    let mut config_option_change = HashMap::new();
    config_option_change.insert(entity, config);
    EventDescriptor {
        component: Default::default(),
        event_data: data_type,
        event_args: ConfigurationOptionEventArgs::Event(ConfigurationOptionChange {
            config_option: config_option_change,
        }, entity),
    }
}