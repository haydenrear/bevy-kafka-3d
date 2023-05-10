use bevy::ecs::query::{QueryEntityError, ReadOnlyWorldQuery, WorldQuery};
use bevy::prelude::{Commands, Component, Entity, EventReader, EventWriter, Interaction, Query, Res, ResMut, Resource};
use bevy::log::info;
use std::marker::PhantomData;
use event_descriptor::{EventArgs, EventData, EventDescriptor};
use event_state::{StateChangeFactory, UpdateStateInPlace};

pub(crate) mod event_descriptor;
pub(crate) mod event_actions;
pub(crate) mod event_state;
pub(crate) mod event_propagation;
pub(crate) mod event_provider;