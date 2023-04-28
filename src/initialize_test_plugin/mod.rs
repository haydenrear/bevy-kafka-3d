use bevy::ecs::component::{ComponentId, Components, StorageType};
use bevy::ecs::storage::Storages;
use bevy::prelude::*;
use bevy::ptr::OwningPtr;
use crate::network::{Layer, NetworkId, Node};

pub struct TestInitialize;

pub(crate) fn add_node_entities(mut commands: Commands) {
    let first = spawn_layer(&mut commands, 0, vec![]);
    let second = spawn_layer(&mut commands, 1, first.1);
    let third = spawn_layer(&mut commands, 2, second.1);
    let fourth = spawn_layer(&mut commands, 3, third.1);
    let fifth = spawn_layer(&mut commands, 4, fourth.1);
}

fn spawn_layer(mut commands: &mut Commands, layer_depth: u8, nodes: Vec<Entity>) -> (Entity, Vec<Entity>) {
    let mut layer = Layer::default();
    layer.name = "this-layer";
    layer.layer_depth = layer_depth;
    layer.network_id = NetworkId::new("id");
    let mut return_nodes = vec![];
    for i in 0..10 {
        return_nodes.push(create_add_node(&mut commands, &mut layer, nodes.clone(), i));
    }

    layer.nodes.iter().for_each(|node| {
        commands.spawn((node.clone(), Transform::default()));
    });

    let layer = commands.spawn((layer, Transform::default()))
        .id();

    (layer, return_nodes)
}

fn create_add_node(commands: &mut Commands, mut layer: &mut Layer, nodes: Vec<Entity>, i: i32) -> Entity {
    let mut node = Node::default();
    node.node_pos = i as u8;
    node.connections.extend(nodes.clone());
    let node_entity = commands.spawn_empty().id();
    node.entity = Some(node_entity);
    layer.nodes.push(node);
    node_entity
}