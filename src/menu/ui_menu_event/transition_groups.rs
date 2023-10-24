use bevy::prelude::Component;

/// In some cases, some events should and should not be propagated to all children. So this determines
/// who to include when building out the tree of events. The TransitionGroup is one of the
/// things that the RetrieveState uses to retrieve the events propagated by the type of
/// EventRetriever.
pub trait TransitionGroup: Component {
}

#[derive(Component, Default, Clone, Debug)]
pub struct PropagateDisplay;

impl TransitionGroup for PropagateDisplay {}

#[derive(Component, Default, Clone, Debug)]
pub struct PropagateVisible;

impl TransitionGroup for PropagateVisible {}

#[derive(Component, Default, Clone, Debug)]
pub struct PropagateScrollable;

impl TransitionGroup for PropagateScrollable {}

#[derive(Component, Default, Clone, Debug)]
pub struct PropagateDraggable;

impl TransitionGroup for PropagateDraggable {}

#[derive(Component, Default, Clone, Debug)]
pub struct PropagateSelect;

impl TransitionGroup for PropagateSelect {}

#[derive(Component, Default, Clone, Debug)]
pub struct PropagateCreateMenu;

impl TransitionGroup for PropagateCreateMenu {}
