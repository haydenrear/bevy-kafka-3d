use bevy::prelude::{BuildChildren, Changed, Children, Color, ColorMaterial, Commands, default, Entity, GlobalTransform, Mesh, Parent, Query, ResMut, shape, SpriteBundle, Transform, Without};
use bevy::asset::Assets;
use bevy::utils::{HashMap, Uuid};
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use bevy_mod_picking::PickableBundle;
use bevy::log::{error, info};
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::shapes;
use bevy::math::Vec2;
use bevy::prelude::shape::Quad;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::draw::{Fill, Stroke};
use crate::metrics::MetricState;
use crate::network::{Layer, LayerType, Network, NetworkId, Node};

pub const NODE_RADIUS: f32 = 30.0;
pub const LAYER_SPACING: f32 = 200.0;
pub const NODE_SPACING: f32 = 70.0;
pub const CONNECTION_THICKNESS: f32 = 2.0;

/// Network created to have ability to inspect Layers to determine how to draw.
/// 1. Add network
/// 2. Use network to update Layer entity with more components specific to type of Neural Network
/// 3. Build particular system, where query includes Layer and also components associated with particular
///    types of neural networks.
pub(crate) fn create_network(
    mut commands: Commands,
    mut layer_query: Query<(&mut Transform, &mut Layer, Entity)>,
    mut network_query: Query<&Network>
) {

    let layers = layer_query.iter()
        .filter(|layer| {
            network_query.iter().all(|n| n.network_id != layer.1.network_id)
        })
        .map(|layer| (layer.1.network_id.clone(), layer))
        .collect::<HashMap<NetworkId, (&Transform, &Layer, Entity)>>();

    if layers.len() == 0 {
        return;
    }

    let mut network: HashMap<NetworkId, Vec<Entity>> = HashMap::new();

    for layer in layers.iter() {
        info!("Creating network {:?}.", layer.0);
        if network.contains_key(layer.0) {
            network.get_mut(layer.0).map(|vec| vec.push(layer.1.2));
        } else {
            network.insert(layer.0.clone(), vec![layer.1.2]);
        }
    }

    for network_to_create in network.into_iter() {
        commands.spawn(Network::new(
            network_to_create.1,
            network_to_create.0
        ));
    }

}

/// Draws fully connected layers.
pub(crate) fn draw_network_initial(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut layer_query: Query<(Entity, &mut Layer, &mut Transform), Changed<Layer>>,
) {
    if layer_query.is_empty() {
        return;
    }

    for layer_tuple in layer_query.iter() {
        draw_layers_and_nodes(&mut commands, &mut materials, &mut meshes, &(layer_tuple.1, layer_tuple.2, layer_tuple.0));
    }
}

/// Draws fully connected layers.
pub(crate) fn update_network(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut layer_query: Query<(&mut Layer, &mut Transform, Entity), Changed<Layer>>,
) {

    for layer_tuple in layer_query.iter() {
        draw_layers_and_nodes(&mut commands, &mut materials, &mut meshes, &layer_tuple);
    }
}

pub(crate) fn draw_node_connections(
    mut commands: Commands,
    mut layer_query: Query<(Entity, &Parent, &mut Transform, &mut Node), Changed<Node>>,
    global_transform_query: Query<&GlobalTransform>
) {
    /// Track if dirty updated somewhere else, in which case do not set not dirty.
    let _ = layer_query.iter()
        .for_each(|layer| {
            // Draw connections between nodes in consecutive layers
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

                    let line = GeometryBuilder::build_as(
                        &shapes::Line(
                            Vec2::new(0.0, layer.2.translation.y),
                            Vec2::new(relative_pos.translation.x, connection_to_make.2.translation.y),
                        ));

                    let line = commands.spawn((
                        ShapeBundle {
                            path: line,
                            ..default()
                        }, Fill::color(Color::BLACK),
                        Stroke::new(Color::BLACK, 1.)
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
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    layer_tuple: &(&'a Layer, &Transform, Entity)
) {


    let layer = layer_tuple.0;
    let layer_entity = layer_tuple.2;

    commands.entity(layer_entity)
        .insert((layer_tuple.0.layer_type.create_mesh(layer_tuple.0, meshes, materials)));

    for node in layer_tuple.0.nodes.iter() {
        let node_entity = node.entity.clone();
        node_entity.map(|node_entity| draw_node(commands, materials, meshes, node, node_entity, layer));
        commands.entity(layer_entity)
            .add_child(node_entity.clone().unwrap());
    }

}

fn draw_node(
    mut commands: &mut Commands,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    node: &Node,
    node_entity: Entity,
    layer: &Layer
) {
    info!("Drawing node!");
    let y = (node.node_pos as f32 * NODE_SPACING) - ((layer.sub_layers.len() as f32 * NODE_SPACING) / 2.0);
    commands.entity(node_entity)
        .insert(node.clone())
        .insert((
            layer.layer_type.draw_node_mesh(y, meshes, materials),
            PickableBundle::default()
        ))
        .insert((PickableBundle::default()))
        .insert((SpriteBundle {
            transform: Transform::from_xyz(0.0, y, 1.0),
            ..Default::default()
        }, PickableBundle::default()));
}