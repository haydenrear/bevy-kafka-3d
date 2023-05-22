use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::prelude::*;
use crate::event::event_actions::{EventsSystem, RetrieveState};
use crate::event::event_descriptor::EventDescriptor;
use crate::event::event_state::{ClickContext, Context};
use crate::menu::{DataType, MenuType, MetricsConfigurationOption};
use crate::menu::config_menu_event::config_event::{ConfigurationOptionChange, ConfigurationOptionEventArgs};
use crate::menu::config_menu_event::config_menu_event_plugin::{MetricsSelfIxnQueryFilter, MetricsSelfQueryFilter};
use crate::menu::ui_menu_event::ui_state_change::{GlobalState, UpdateGlobalState};

#[derive(Default, Resource, Debug)]
pub struct ConfigOptionActionStateRetriever<T: Component>
    where T: Component + Send + Sync + Default + Clone + Debug + 'static
{
    phantom: PhantomData<T>,
}

impl <T> UpdateGlobalState<MetricsSelfQueryFilter<T>,MetricsSelfIxnQueryFilter<T>>
for ConfigOptionActionStateRetriever<T>
    where T: Component + Send + Sync + Default + Clone + Debug + 'static {
}

#[derive(Default, Resource, Debug)]
pub struct NetworkMenuResultBuilder {
    pub(crate) network_parent_entity: Option<Entity>,
    pub(crate) network_menu_config_option: Option<Entity>
}

#[derive(Default, Resource, Debug)]
pub struct GraphMenuResultBuilder {
    pub(crate) graph_parent_entity: Option<Entity>,
    pub(crate) graph_menu_config_option: Option<Entity>,
}

impl Context for NetworkMenuResultBuilder {}

impl <T> ClickContext<MetricsSelfQueryFilter<T>, MetricsSelfIxnQueryFilter<T>>
for NetworkMenuResultBuilder
where T: Component + Send + Sync + Default + Clone + Debug + 'static
{
    fn clicked(&mut self, entity: Entity) {
    }

    fn un_clicked(&mut self) {
    }

    fn cursor(&mut self, global_stat: &mut ResMut<GlobalState>) {
    }
}

impl<T: Component + Send + Sync + Default + Clone + Debug + 'static> EventsSystem<
    ConfigOptionActionStateRetriever<T>, ConfigurationOptionEventArgs<T>,
    DataType, MetricsConfigurationOption<T>, MetricsConfigurationOption<T>, NetworkMenuResultBuilder,
    // self query
    (Entity, &MetricsConfigurationOption<T>),
    // self filter
    MetricsSelfQueryFilter<T>,
    // parent query
    (Entity, &MetricsConfigurationOption<T>),
    // parent filter
    (With<MetricsConfigurationOption<T>>),
    // interaction filter
    MetricsSelfIxnQueryFilter<T>
> for ConfigOptionActionStateRetriever<T> {}

impl<T: Component + Send + Sync + Default + Clone + Debug + 'static> RetrieveState<
    ConfigurationOptionEventArgs<T>,
    DataType,
    MetricsConfigurationOption<T>,
    MetricsConfigurationOption<T>,
    NetworkMenuResultBuilder,
    (Entity, &MetricsConfigurationOption<T>),
    (Entity, &MetricsConfigurationOption<T>),
    MetricsSelfQueryFilter<T>,
    (With<MetricsConfigurationOption<T>>),
>
for ConfigOptionActionStateRetriever<T>
{
    fn create_event(
        commands: &mut Commands,
        entity: Entity,
        mut context: &mut ResMut<NetworkMenuResultBuilder>,
        self_query: &Query<
            (Entity, &MetricsConfigurationOption<T>),
            (With<MetricsConfigurationOption<T>>)
        >,
        propagation_query: &Query<
            (Entity, &MetricsConfigurationOption<T>),
            (With<MetricsConfigurationOption<T>>)
        >,
    ) -> (
        Vec<EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>>
    )
    {
        let mut event_descriptors = vec![];
        info!("Here is ctx: {:?}", context);
        Self::set_menu_events(entity, context, self_query, &mut event_descriptors);
        event_descriptors
    }
}

impl<T> ConfigOptionActionStateRetriever<T>
where T: Component + Send + Sync + Default + Clone + Debug + 'static
{
    fn set_menu_events(
        entity: Entity,
        mut context: &mut ResMut<NetworkMenuResultBuilder>,
        self_query: &Query<(Entity, &MetricsConfigurationOption<T>), With<MetricsConfigurationOption<T>>>,
        mut event_descriptors: &mut Vec<EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>>,
    ) {
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
                                entity,
                                config_menu,
                                DataType::Deselected,
                                Visibility::Hidden,
                                None,
                            );
                        } else if let DataType::Deselected = data_type {
                            let config_menu = MetricsConfigurationOption::GraphMenu(
                                PhantomData::<T>::default(),
                                DataType::Selected,
                                key,
                                MenuType::Graph,
                            );
                            create_add_events(&mut event_descriptors,
                                              entity,
                                              config_menu,
                                              DataType::Selected,
                                              Visibility::Visible,
                                              None,
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
                                              entity,
                                              config_menu,
                                              DataType::Deselected,
                                              Visibility::Hidden,
                                              context.network_parent_entity,
                            );
                        } else if let DataType::Deselected = data_type {
                            let config_menu = MetricsConfigurationOption::NetworkMenu(
                                PhantomData::<T>::default(),
                                DataType::Selected,
                                key,
                                MenuType::Network,
                            );
                            create_add_events(&mut event_descriptors,
                                              entity,
                                              config_menu,
                                              DataType::Selected,
                                              Visibility::Visible,
                                              context.network_parent_entity,
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

/// I need a way to interface between the two systems. So the events that have an effect on the UI
/// should then create events that affect the UI. And all events that effect the UI should go through
/// the UI system.
fn create_add_events<T>(
    mut event_descriptors: &mut Vec<EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>>,
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
    event_descriptors.push(event_descriptor);
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