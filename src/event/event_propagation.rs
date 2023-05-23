use bevy::prelude::{Commands, Component, Entity, error, EventReader, info, Visibility};
use bevy::ui::{Display, Style};

/// Determines where to get the starting state from, which determines the next state. For instance,
/// if a child is swapping from visible to invisible, and the parent is swapping, then in order so
/// that they won't swap out of sync, you use starting state of one to determine next state of both.
#[derive(Clone, Debug)]
pub enum Relationship {
    Child,
    Parent,
    SelfState,
    EachSelfState,
    Sibling,
    SiblingChild,
    Other(f32),
    VisibleState(Display)
}

