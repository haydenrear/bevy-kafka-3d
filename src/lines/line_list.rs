use bevy::math::Vec3;
use bevy::{prelude::*};
use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::{MeshVertexBufferLayout, PrimitiveTopology},
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};
use bevy::reflect::TypeUuid;

pub(crate) fn create_3d_line(
    line_list: LineList,
    line_material: LineMaterial
) -> (Mesh, Mesh, LineMaterial) {(
        Mesh::from(line_list.clone()),
        Mesh::from(line_list.to_line_strip()),
        line_material
)}


/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
pub struct LineStrip {
    pub points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        // This tells wgpu that the positions are a list of points
        // where a line will be drawn between each consecutive point
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, line.points);
        mesh
    }
}

#[derive(TypePath, Default, AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "050ce6ac-080a-4d8c-b6b5-b5bab7560d8f"]
pub struct LineMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
    pub thickness: f32,
}

impl LineList {
    pub fn to_line_strip(&self) -> LineStrip {
        let mut lines = vec![];
        for (from, to) in self.lines.iter() {
            lines.push(*from);
            lines.push(*to);
        }
        return LineStrip {
            points: lines,
        }
    }
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        // This tells wgpu that the positions are list of lines
        // where every pair is a start and end point
        let vertices: Vec<_> = line.lines.iter()
            .flat_map(|(a, b)| [a.to_array(), b.to_array()]).collect();
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices );
        mesh
    }
}


fn compute_line_normal(start: &Vec3, end: &Vec3) -> Vec3 {
    let direction = *end - *start;
    let up = Vec3::new(0.0, 1.0, 0.0);
    let perp = if direction.cross(up).length() > 0.0001 {
        direction.cross(up).normalize()
    } else {
        direction.cross(Vec3::new(1.0, 0.0, 0.0)).normalize()
    };
    let normal = direction.cross(perp).normalize();
    normal
}

