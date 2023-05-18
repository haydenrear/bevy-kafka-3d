use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::input::mouse::MouseScrollUnit;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use crate::cursor_adapter::CursorResource;
use crate::event::event_actions::{ClickWriteEvents, RetrieveState};
use crate::event::event_descriptor::EventDescriptor;
use crate::event::event_propagation::PropagateComponentEvent;
use crate::event::event_state::{ClickContext, Context};
use crate::menu::{ConfigurationOptionComponent, ConfigurationOptionEnum, DataType, MenuType, MetricsConfigurationOption};
use crate::menu::config_menu_event::config_event::{ConfigurationOptionChange, ConfigurationOptionEventArgs};
use crate::menu::config_menu_event::config_menu_event_plugin::{MetricsSelfIxnQueryFilter, MetricsSelfQueryFilter};
use crate::menu::menu_resource::MENU;
use crate::menu::ui_menu_event::ui_state_change::{GlobalState, UpdateGlobalState};
use crate::network::{Network, Node};
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

#[derive(Default, Resource, Debug)]
pub struct ConfigOptionActionStateRetriever<T: Component>
    where T: Component + Send + Sync + Default + Clone + Debug + 'static
{
    phantom: PhantomData<T>,
}

impl <T> UpdateGlobalState<MetricsSelfQueryFilter<T>,MetricsSelfIxnQueryFilter<T>> for ConfigOptionActionStateRetriever<T>
    where T: Component + Send + Sync + Default + Clone + Debug + 'static
{
}

#[derive(Default, Resource, Debug)]
pub struct ConfigOptionContext {
    pub(crate) graph_parent_entity: Option<Entity>,
    pub(crate) network_entity: Option<Entity>,
}

impl Context for ConfigOptionContext {}

impl <T> ClickContext<MetricsSelfQueryFilter<T>, MetricsSelfIxnQueryFilter<T>>
for ConfigOptionContext
where T: Component + Send + Sync + Default + Clone + Debug + 'static
{
    fn clicked(&mut self) {
    }

    fn un_clicked(&mut self) {
    }

    fn cursor(&mut self, global_stat: &mut ResMut<GlobalState>) {
    }
}

impl<T: Component + Send + Sync + Default + Clone + Debug + 'static> ClickWriteEvents<
    ConfigOptionActionStateRetriever<T>, ConfigurationOptionEventArgs<T>,
    DataType, MetricsConfigurationOption<T>, ConfigOptionContext,
    // self query
    (Entity, &MetricsConfigurationOption<T>),
    // self filter
    MetricsSelfQueryFilter<T>,
    // parent query
    (Entity, &Parent, &MetricsConfigurationOption<T>),
    // parent filter
    (With<Parent>, With<MetricsConfigurationOption<T>>),
    // interaction filter
    MetricsSelfIxnQueryFilter<T>
> for ConfigOptionActionStateRetriever<T> {}

impl<T: Component + Send + Sync + Default + Clone + Debug + 'static> RetrieveState<
    ConfigurationOptionEventArgs<T>,
    DataType,
    MetricsConfigurationOption<T>,
    ConfigOptionContext,
    (Entity, &MetricsConfigurationOption<T>),
    (Entity, &Parent, &MetricsConfigurationOption<T>),
    MetricsSelfQueryFilter<T>,
    (With<Parent>, With<MetricsConfigurationOption<T>>),
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
        propagation_query: &Query<
            (Entity, &Parent, &MetricsConfigurationOption<T>),
            (With<Parent>, With<MetricsConfigurationOption<T>>)
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