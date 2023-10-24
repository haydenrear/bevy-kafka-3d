use bevy::asset::Asset;
use bevy::math::Vec3;
use bevy::pbr::{Material, MaterialPipeline, MaterialPipelineKey};
use bevy::render::render_resource::{AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError};
use bevy::render::mesh::{Indices, MeshVertexBufferLayout, PrimitiveTopology, VertexAttributeValues};
use bevy::prelude::{Assets, Color, default, Mesh, ResMut};
use bevy::reflect::{TypePath, TypeUuid};
use bevy_polyline::prelude::{Polyline, PolylineBundle, PolylineMaterial};

pub(crate) fn create_3d_line(
    line_list: LineList,
    mut polylines: &mut ResMut<Assets<Polyline>>,
    mut polyline_materials: &mut ResMut<Assets<PolylineMaterial>>,
) -> PolylineBundle {
    PolylineBundle {
        polyline: polylines.add(Polyline {
            vertices: line_list.lines.into_iter().flat_map(|(a, b)| vec![a, b]).collect()
        }),
        material: polyline_materials.add(PolylineMaterial {
            color: line_list.color,
            width: line_list.thickness,
            ..default()
        }),
        ..default()
    }
}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub(crate) color: Color,
    pub(crate) lines: Vec<(Vec3, Vec3)>,
    pub(crate) thickness: f32,
}
