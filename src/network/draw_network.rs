use std::collections::HashSet;
use bevy::prelude::{Changed, ClearColor, Commands, default, Entity, Mesh, Query, Res, ResMut, SpriteBundle, Visibility};
use bevy::asset::Assets;
use bevy::pbr::{MaterialMeshBundle, PbrBundle, StandardMaterial};
use bevy_transform::components::{GlobalTransform, Transform};
use bevy::log::info;
use bevy_mod_picking::PickableBundle;
use std::marker::PhantomData;
use bevy::hierarchy::{BuildChildren, Parent};
use bevy::math::Vec3;
use crate::lines::line_list::{create_3d_line, LineList, LineMaterial};
use crate::menu::{DataType, MetricsConfigurationOption};
use crate::menu::config_menu_event::interaction_config_event_writer::NetworkMenuResultBuilder;
use crate::menu::menu_resource::VARIANCE;
use crate::menu::ui_menu_event::transition_groups::PropagateVisible;
use crate::network::{Layer, Network, Node};
use crate::util;

pub const NODE_RADIUS: f32 = 1.0;
pub const LAYER_SPACING: f32 = 30.0;
pub const NODE_SPACING: f32 = 10.0;
pub const CONNECTION_THICKNESS: f32 = 2.0;

/// Network created to have ability to inspect Layers to determine how to draw.
/// 1. Add network
/// 2. Use network to update Layer entity with more components specific to type of Neural Network
/// 3. Build particular system, where query includes Layer and also components associated with particular
///    types of neural networks.
pub(crate) fn create_network(
    mut commands: Commands,
    mut context: ResMut<NetworkMenuResultBuilder>,
    mut layer_query: Query<(&mut Transform, &mut Layer, Entity), Changed<Layer>>,
    mut network_query: Query<&mut Network>
) {

    if !layer_query.is_empty() {
        info!("Adding layer to network.");
    } else {
        return;
    }

    let grouped_by_network_id = util::group_by_key(
        layer_query.iter()
            .map(|(entity, layer, transform)| {
                (layer.network_id.clone(), transform)
            }).collect()
    );

    let mut existing = HashSet::new();

    for (network_id, layer) in grouped_by_network_id.iter() {
        for mut network in network_query.iter_mut() {
            if network.network_id.network_id == network_id.network_id {
                for layer in layer.iter() {
                    network.layers.insert(*layer);
                    existing.insert(network_id.clone());
                }
            }
        }
    }

    info!("{:?} are existing networks and {:?} are grouped by network id.", &existing, &grouped_by_network_id);

    for (network_id, layers) in grouped_by_network_id.into_iter() {
        if !existing.contains(&network_id) {
            let new_network = Network::new(layers.clone(), network_id);
            let mut created_network = commands.spawn((
                new_network,
                PbrBundle::default(),
                PropagateVisible::default()
            ));
            let hidden_network = created_network
                .insert(Visibility::Hidden);
            let network = hidden_network
                .push_children(layers.into_iter().collect::<Vec<Entity>>().as_slice());
            let network = network.id();
            info!("Setting network entity: {:?}.", network);
            context.network_parent_entity = Some(network);
        }
    }


}

/// Draws fully connected layers.
pub(crate) fn draw_network_initial(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut layer_query: Query<(Entity, &mut Layer, &mut Transform), Changed<Layer>>,
    color: Res<ClearColor>,
) {
    if layer_query.is_empty() {
        return;
    }

    for layer_tuple in layer_query.iter() {
        draw_layers_and_nodes(&mut commands, &mut materials, &mut meshes, &(layer_tuple.1, layer_tuple.2, layer_tuple.0), &color);
    }
}


/// Draws fully connected layers.
pub(crate) fn update_network(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut layer_query: Query<(&mut Layer, &mut Transform, Entity), Changed<Layer>>,
    color: Res<ClearColor>
) {

    for layer_tuple in layer_query.iter() {
        draw_layers_and_nodes(&mut commands, &mut materials, &mut meshes, &layer_tuple, &color);
    }
}


pub(crate) fn draw_node_connections(
    mut commands: Commands,
    mut layer_query: Query<(Entity, &Parent, &mut Transform, &mut Node), Changed<Node>>,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    global_transform_query: Query<&GlobalTransform>
) {
    /// Track if dirty updated somewhere else, in which case do not set not dirty.
    let _ = layer_query.iter()
        .for_each(|layer| {
            // Draw connections between nodes in consecutive layers
            // The connections for two layers are owned by the receiving layer.
            info!("drawing!");
            let parent_pos = global_transform_query.get(layer.1.get())
                .unwrap();
            let lines = layer.3.connections.iter()
                .flat_map(|node_cxn| {
                    return layer_query.get(node_cxn.clone())
                        .ok()
                        .into_iter()
                        .collect::<Vec<(Entity, &Parent, &Transform, &Node)>>()
                })
                .map(|connection_to_make| {
                    let connection_parent_pos = global_transform_query.get(connection_to_make.1.get())
                        .unwrap();

                    let relative_pos = parent_pos.compute_matrix().inverse()
                        * connection_parent_pos.compute_matrix();

                    let relative_pos = Transform::from_matrix(relative_pos);

                    let mesh = create_3d_line(LineList {
                        lines: vec![(
                            Vec3::new(0.0, layer.2.translation.y, 1.0),
                            Vec3::new(relative_pos.translation.x, connection_to_make.2.translation.y, 1.0)
                        )],
                        thickness: 0.5,
                    }, LineMaterial::default());

                    let line = commands.
                        spawn((
                            MaterialMeshBundle {
                                mesh: meshes.add(mesh.0),
                                material: materials.add(mesh.1),
                                ..default()
                            }
                        ));

                    line.id()
                })
                .collect::<Vec<Entity>>();

            commands.entity(layer.1.get())
                .insert_children(0, &lines);
        });

}

fn draw_layers_and_nodes<'a>(
    mut commands: &mut Commands,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    layer_tuple: &(&'a Layer, &Transform, Entity),
    color: &Res<ClearColor>
) {


    let layer = layer_tuple.0;
    let layer_entity = layer_tuple.2;

    commands.entity(layer_entity)
        .insert((layer_tuple.0.layer_type.create_mesh(layer_tuple.0, meshes, materials, color)));


    for node in layer_tuple.0.nodes.iter() {
        if node.entity.is_some() {
            let node_entity = node.entity.clone();
            node_entity.map(|node_entity| draw_node(commands, materials, meshes, node, node_entity, layer));
            commands.entity(layer_entity)
                .add_child(node_entity.clone().unwrap());
        }
    }

}

fn draw_node(
    mut commands: &mut Commands,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    node: &Node,
    node_entity: Entity,
    layer: &Layer
) {
    info!("Drawing node!");
    let mut y = (node.node_pos as f32 * NODE_SPACING) - ((layer.sub_layers.len() as f32 * NODE_SPACING) / 2.0);
    let value = (NODE_RADIUS * 2.0 * layer.nodes.len() as f32 + NODE_SPACING * (layer.nodes.len() - 1) as f32) / 2 as f32;
    commands.entity(node_entity)
        .insert(node.clone())
        .insert((
            layer.layer_type.draw_node_mesh(y, meshes, materials),
            PickableBundle::default()
        ))
        .insert(PickableBundle::default())
        .insert((
                    SpriteBundle {
                    transform: Transform::from_xyz(0.0, y - value + LAYER_SPACING / 4.0, 1.0),
                    ..Default::default()
                },
                PickableBundle::default()
        ))
        .with_children(|child| {
            child.spawn(MetricsConfigurationOption::Variance(PhantomData::<Node>::default(), DataType::Selected, VARIANCE));
        });
}
