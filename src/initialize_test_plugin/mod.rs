use bevy::ecs::component::{ComponentId, Components, StorageType};
use bevy::ecs::storage::Storages;
use bevy::prelude::*;
use bevy::ptr::OwningPtr;
use crate::network::{Layer, Node};

pub struct TestInitialize;

pub(crate) fn add_node_entities(mut commands: Commands) {
    let first = spawn_layer(&mut commands, 0, vec![]);
    let first = spawn_layer(&mut commands, 1, first);
    let first = spawn_layer(&mut commands, 2, first);
    let first = spawn_layer(&mut commands, 3, first);
    let first = spawn_layer(&mut commands, 4, first);
}

fn spawn_layer(mut commands: &mut Commands, layer_depth: u8, nodes: Vec<Entity>) -> Vec<Entity> {
    let mut layer = Layer::default();
    layer.name = "this-layer";
    layer.layer_depth = layer_depth;
    let mut nodes = vec![];
    for _ in 0..10 {
        nodes.push(create_add_node(&mut commands, &mut layer));
    }

    layer.nodes.iter().for_each(|node| {
        commands.spawn((node.clone(), Transform::default()));
    });

    commands.spawn((layer, Transform::default()));

    nodes
}

fn create_add_node(commands: &mut Commands, mut layer: &mut Layer, nodes: Vec<Entity>) -> Entity {
    let mut node = Node::default();
    node.connections.extend(nodes);
    let node_entity = commands.spawn_empty().id();
    node.entity = Some(node_entity);
    layer.nodes.push(node);
    node_entity
}