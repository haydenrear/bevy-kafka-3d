/// An event that includes an interaction with an entity that contains a configuration option component
/// happens, and then the configurations are updated.
/// The event writer will read

pub(crate) mod interaction_config_event_writer;
pub(crate) mod config_event;
pub(crate) mod config_event_reader;
pub(crate) mod network_component_interaction_system;
pub(crate) mod config_menu_event_plugin;
pub(crate) mod config_event_state_change;