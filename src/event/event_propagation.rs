use crate::event::event_state::StateChange;

#[derive(Clone, Debug)]
pub enum ChangePropagation {
    // Include self as parent and any children of parent
    ParentToChild(StartingState),
    // Include self as child and parent of self
    ChildToParent(StartingState),
    // Include self only
    SelfChange(StartingState),
    // Include children only
    Children(StartingState),
    // Include parent only
    Parent(StartingState),
    // Propagate event to specific Id's
    CustomPropagation {
        to: Vec<f32>,
        // starting state
        from: StartingState
    }
}

impl ChangePropagation {
    pub(crate) fn get_starting_state(&self) -> &StartingState {
        match self {
            ChangePropagation::ParentToChild(starting) => {
                starting
            }
            ChangePropagation::ChildToParent(starting) => {
                starting
            }
            ChangePropagation::SelfChange(starting) => {
                starting
            }
            ChangePropagation::Children(starting) => {
                starting
            }
            ChangePropagation::Parent(starting) => {
                starting
            }
            ChangePropagation::CustomPropagation { to , from} => {
                from
            }
        }
    }
}

impl ChangePropagation {
    pub(crate) fn includes_parent(&self) -> bool {
        match self {
            ChangePropagation::ParentToChild(_) => {
                true
            }
            ChangePropagation::ChildToParent(_) => {
                true
            }
            ChangePropagation::SelfChange(_) => {
                false
            }
            ChangePropagation::Children(_) => {
                false
            }
            ChangePropagation::Parent(_) => {
                true
            }
            ChangePropagation::CustomPropagation { .. } => {
                false
            }
        }
    }

    pub(crate) fn includes_self(&self) -> bool {
        match self {
            ChangePropagation::ParentToChild(_) => {
                true
            }
            ChangePropagation::ChildToParent(_) => {
                true
            }
            ChangePropagation::SelfChange(_) => {
                true
            }
            ChangePropagation::Children(_) => {
                false
            }
            ChangePropagation::Parent(_) => {
                false
            }
            ChangePropagation::CustomPropagation { .. } => {
                false
            }
        }
    }

    pub(crate) fn includes_children(&self) -> bool {
        match self {
            ChangePropagation::ParentToChild(_) => {
                true
            }
            ChangePropagation::ChildToParent(_) => {
                false
            }
            ChangePropagation::SelfChange(_) => {
                false
            }
            ChangePropagation::Children(_) => {
                true
            }
            ChangePropagation::Parent(_) => {
                false
            }
            ChangePropagation::CustomPropagation { .. } => {
                false
            }
        }
    }
}

/// Determines where to get the starting state from, which determines the next state. For instance,
/// if a child is swapping from visible to invisible, and the parent is swapping, then in order so
/// that they won't swap out of sync, you use starting state of one to determine next state of both.
#[derive(Clone, Debug)]
pub enum StartingState {
    Child,
    Parent,
    SelfState,
    Other(f32)
}

impl StateChange {

    pub fn propagation(&self) -> Option<&ChangePropagation> {
        match self {
            StateChange::ChangeComponentColor(_, change_type) => {
                Some(change_type)
            }
            StateChange::ChangeComponentStyle(_, change_type) => {
                Some(change_type)
            }
            StateChange::None => {
                None
            }
        }
    }

}
