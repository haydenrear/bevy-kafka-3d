use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::HashMap;
use bevy_mod_picking::PickableBundle;
use bevy_prototype_lyon::draw::{Fill, Stroke};
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::GeometryBuilder;
use bevy_prototype_lyon::shapes;
use crate::component_interaction::{Highlightable};
use crate::metrics::{LayerMetrics, Metric, MetricState};

#[derive(Default, Component, Clone)]
pub struct Node {
    metrics: Metric,
    pub(crate) entity: Option<Entity>
}

#[derive(Default, Component)]
pub struct Layer {
    pub(crate) nodes: Vec<Node>,
    pub(crate) name: &'static str,
    pub(crate) layer_type: LayerType,
    pub(crate) layer_depth: u8
}

#[derive(Default)]
pub enum LayerType {
    #[default]
    TFormer,
    FullyConnected,
    Normalization
}

#[derive(Default)]
pub struct Network {
    layers: Vec<Layer>
}

#[derive(Default)]
pub struct MetaNetwork {
    network_connections: HashMap<Entity, Entity>
}

// You may need to adjust these constants for the size and spacing of nodes and layers
const NODE_RADIUS: f32 = 30.0;
const LAYER_SPACING: f32 = 200.0;
const NODE_SPACING: f32 = 70.0;
const CONNECTION_THICKNESS: f32 = 2.0;

pub(crate) fn draw_layers(
    mut commands: Commands,
    mut app_state: ResMut<MetricState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut layer_query: Query<(&mut Transform, &mut Layer, Entity)>,
) {

    let mut metrics = app_state.as_mut();


    let node_material = materials.add(Color::rgb(0.7, 0.7, 0.7).into());
    let connection_material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());

    let mut prev_layer: Option<&Layer> = None;
    let mut layer_query = layer_query.iter().enumerate().collect::<Vec<(usize, (&Transform, &Layer, Entity))>>();
    layer_query
        .sort_by(|first, second| {
            return first.1.1.layer_depth.cmp(&second.1.1.layer_depth);
        });

    for (layer_index, layer_tuple) in layer_query.iter() {

        let layer = layer_tuple.1;
        let layer_entity = layer_tuple.2;

        if *layer_index != 0 {
            prev_layer = Some(layer_tuple.1);
        }

        if !metrics.metrics.contains_key(&layer_entity) {
            let entities = layer.nodes.iter()
                .map(|n| (n.entity.unwrap(), Metric::default()))
                .collect::<HashMap<Entity, Metric>>();
            metrics.metrics.insert(layer_entity.clone(), LayerMetrics::new(entities));
        }

        metrics.metrics.get_mut(&layer_entity).map(|layer| {

            if layer.dirty {

                for (node_index, node) in layer_tuple.1.nodes.iter().enumerate() {
                    let node_entity = node.entity.clone();
                    let layer_len = layer.metrics.len();
                    layer.metrics.get_mut(&node_entity.unwrap()).as_mut().map(|metric| {

                        if metric.dirty {
                            let x = *layer_index as f32 * LAYER_SPACING;
                            let y = (node_index as f32 * NODE_SPACING) - ((layer_len as f32 * NODE_SPACING) / 2.0);
                            // Draw node as circle
                            node_entity.map(|node_entity| {
                                commands.entity(node_entity)
                                    .insert(node.clone())
                                    .insert((MaterialMesh2dBundle {
                                        mesh: meshes.add(shape::Circle::new(30.).into()).into(),
                                        material: materials.add(ColorMaterial::from(Color::PURPLE)),
                                        transform: Transform::from_xyz(x, y, 0.0),
                                        ..default()
                                    }, PickableBundle::default()))
                                    .insert((Highlightable::default(), PickableBundle::default()))
                                    .insert((SpriteBundle {
                                        transform: Transform::from_xyz(x, y, 0.0),
                                        ..Default::default()
                                    }, PickableBundle::default()));
                            });

                            // Draw connections between nodes in consecutive layers
                            if *layer_index > 0 {
                                for (prev_node_index, _prev_node_value) in prev_layer.unwrap().nodes.iter().enumerate() {
                                    info!("adding line");
                                    let prev_x = (*layer_index as f32 - 1.0) * LAYER_SPACING;
                                    let prev_y = (prev_node_index as f32 * NODE_SPACING)
                                        - ((prev_layer.unwrap().nodes.len() as f32 * NODE_SPACING) / 2.0);

                                    let line = GeometryBuilder::build_as(
                                        &shapes::Line(
                                            Vec2::new(x, y), Vec2::new(prev_x, prev_y),
                                        ));

                                    commands.spawn((ShapeBundle {
                                            path: line,
                                            ..default()
                                        }, Fill::color(Color::BLACK),
                                        Stroke::new(Color::BLACK, 1.)
                                    ));

                                }
                            }
                        }

                        metric.dirty = false;

                    });
                }
            }

            layer.dirty = false;
        });

    }
}

