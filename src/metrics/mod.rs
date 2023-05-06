use std::default::{default, Default};
use bevy::prelude::{BuildChildren, Children, Commands, Component, Entity, info, Query, Res, ResMut, Resource};
use bevy::utils::HashMap;
use bevy_prototype_lyon::prelude::tess::geom::Transform;
use crate::network::Node;

/// Provide metrics for nodes and layers
pub(crate) mod network_metrics;
/// Retrieve the network loss and feed it into graph loss resource
pub(crate) mod network_loss;