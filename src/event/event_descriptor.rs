use bevy::prelude::{Component};
use std::marker::PhantomData;

/// Event data generated from the event writer, added in order to determine next state.
pub trait EventArgs: Send + Sync {
}

/// Event data passed into the source.
pub trait EventData: Send + Sync {
}

#[derive(Debug)]
/// The description of the event, propagated as the event. Contains type data about which component
/// the event will be used to update, the original data passed in, and the arguments added by the
/// event writer.
pub struct EventDescriptor<EventDataT, EventArgsT, ComponentT>
where
    EventDataT: EventData,
    EventArgsT: EventArgs,
    ComponentT: Component + Send + Sync + 'static
{
    /// The component for which the state will be updated
    pub(crate) component: PhantomData<ComponentT>,
    /// Contains all data needed to update the state
    pub(crate) event_data: EventDataT,
    pub(crate) event_args: EventArgsT
}
