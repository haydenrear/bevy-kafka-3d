use bevy::ecs::component::{ComponentId, Components, StorageType};
use bevy::ecs::storage::Storages;
use bevy::prelude::*;
use bevy::ptr::OwningPtr;
use crate::network::{Layer, Node};

pub struct TestInitialize;

pub(crate) fn add_node_entities(mut commands: Commands) {
    spawn_layer(&mut commands, 0);
    spawn_layer(&mut commands, 1);
    spawn_layer(&mut commands, 2);
    spawn_layer(&mut commands, 3);
    spawn_layer(&mut commands, 4);
}

fn spawn_layer(mut commands: &mut Commands, layer_depth: u8) {
    let mut layer = Layer::default();
    layer.name = "this-layer";
    layer.layer_depth = layer_depth;
    for _ in 0..10 {
        create_add_node(&mut commands, &mut layer);
    }

    layer.nodes.iter().for_each(|node| {
        commands.spawn((node.clone(), Transform::default()));
    });

    commands.spawn((layer, Transform::default()));
}

fn create_add_node(commands: &mut Commands, layer: &mut Layer) {
    let mut node = Node::default();
    let node_entity = commands.spawn_empty().id();
    node.entity = Some(node_entity);
    layer.nodes.push(node);
}