use bevy::prelude::{BuildChildren, Color, ColorMaterial, Commands, default, Entity, GlobalTransform, Mesh, Parent, Query, ResMut, shape, SpriteBundle, Transform};
use bevy::asset::Assets;
use bevy::utils::HashMap;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use bevy_mod_picking::PickableBundle;
use bevy::log::info;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::shapes;
use bevy::math::Vec2;
use bevy::prelude::shape::Quad;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::draw::{Fill, Stroke};
use crate::component_interaction::Highlightable;
use crate::metrics::{LayerMetrics, Metric, MetricState};
use crate::network::{Layer, Node};

const NODE_RADIUS: f32 = 30.0;
const LAYER_SPACING: f32 = 200.0;
const NODE_SPACING: f32 = 70.0;
const CONNECTION_THICKNESS: f32 = 2.0;


/// Draws fully connected layers.
pub(crate) fn draw_network(
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
    let mut layer_query = layer_query.iter()
        .enumerate()
        .collect::<Vec<(usize, (&Transform, &Layer, Entity))>>();

    layer_query.sort_by(|first, second| {
            return first.1.1.layer_depth.cmp(&second.1.1.layer_depth);
        });

    for (layer_index, layer_tuple) in layer_query.iter() {
        draw_fc_layer(&mut commands, &mut materials, &mut meshes, &mut metrics, prev_layer, layer_index, layer_tuple);
    }
}

pub fn draw_node_connections(
    mut commands: Commands,
    mut app_state: ResMut<MetricState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut layer_query: Query<(Entity, &Parent, &mut Transform, &mut Node)>,
    global_transform_query: Query<&GlobalTransform>
) {
    for layer in layer_query.iter_mut() {
        info!("parent found!");
        let parent_transform = global_transform_query.get(layer.1.get())
            .unwrap();
        info!("parent transform {}", parent_transform.translation().x);
    }
}

fn draw_fc_layer<'a>(
    mut commands: &mut Commands,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    metrics: &mut MetricState,
    mut prev_layer: Option<&'a Layer>,
    layer_index: &usize,
    layer_tuple: &(&Transform, &'a Layer, Entity)
) {
    let layer = layer_tuple.1;
    let layer_entity = layer_tuple.2;

    commands.entity(layer_entity)
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Quad::new(Vec2::new(NODE_RADIUS + 20.0, NODE_RADIUS * 2.0 * layer.nodes.len() as f32 + 100.0)))).into(),
            transform: Transform::from_xyz(*layer_index as f32 * LAYER_SPACING, 0.0, 0.0),
            material: materials.add(ColorMaterial::from(Color::BEIGE)),
            ..default()
        });

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
                                    transform: Transform::from_xyz(0.0, y, 1.0),
                                    ..default()
                                }, PickableBundle::default()))
                                .insert((Highlightable::default(), PickableBundle::default()))
                                .insert((SpriteBundle {
                                    transform: Transform::from_xyz(0.0, y, 1.0),
                                    ..Default::default()
                                }, PickableBundle::default()));
                        });

                        // Draw connections between nodes in consecutive layers
                        if *layer_index > 0 {
                            let mut lines: Vec<Entity> = vec![];
                            for (prev_node_index, _prev_node_value) in prev_layer.unwrap().nodes.iter().enumerate() {
                                info!("adding line");

                                let prev_y = (prev_node_index as f32 * NODE_SPACING)
                                    - ((prev_layer.unwrap().nodes.len() as f32 * NODE_SPACING) / 2.0);

                                let line = GeometryBuilder::build_as(
                                    &shapes::Line(
                                        Vec2::new(0.0, y),
                                        Vec2::new(-LAYER_SPACING, prev_y),
                                    ));

                                let line = commands.spawn((
                                    ShapeBundle {
                                        path: line,
                                        ..default()
                                    }, Fill::color(Color::BLACK),
                                    Stroke::new(Color::BLACK, 1.)
                                ));
                                lines.push(line.id().clone());
                            }

                            commands.entity(layer_entity)
                                .insert_children(0, &lines);
                        }
                    }

                    commands.entity(layer_entity)
                        .add_child(node_entity.clone().unwrap());

                    metric.dirty = false;
                });
            }
        }

        layer.dirty = false;
    });
}
