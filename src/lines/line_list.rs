use bevy::math::Vec3;
use bevy::pbr::{Material, MaterialPipeline, MaterialPipelineKey};
use bevy::render::render_resource::{AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError};
use bevy::render::mesh::{Indices, MeshVertexBufferLayout, PrimitiveTopology, VertexAttributeValues};
use bevy::prelude::{Color, Mesh};
use bevy::reflect::TypeUuid;

pub(crate) fn create_3d_line(line_list: LineList, line_material: LineMaterial) -> (Mesh, LineMaterial)  {
    (Mesh::from(line_list), line_material)
}

#[derive(Default, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "050ce6ac-080a-4d8c-b6b5-b5bab7560d8f"]
pub struct LineMaterial {
    #[uniform(0)]
    pub(crate) color: Color,
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
        descriptor.primitive.cull_mode = None;
        descriptor.primitive.polygon_mode = PolygonMode::Fill;
        Ok(())
    }
}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
    pub thickness: f32,
}


impl From<LineList> for Mesh {
    fn from(line_list: LineList) -> Self {
        let mut indices = Vec::new();
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut vertex_offset: u32 = 0;

        for (start, end) in line_list.lines {
            let mesh = create_thick_line_mesh(&[(start, end)], line_list.thickness);
            let new_vertices = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

            // Adjust the indices to take into account the vertex_offset
            let new_vertices = match mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap() {
                VertexAttributeValues::Float32x3(data) => data.clone(),
                _ => panic!("Unexpected attribute format"),
            };

            let vertices_len = new_vertices.len();

            vertices.extend(new_vertices);

            let adjusted_indices: Vec<u32> = mesh.indices()
                .unwrap()
                .iter()
                .map(|i| (i as u32 + vertex_offset) as u32)
                .collect();

            indices.extend(adjusted_indices);

            vertex_offset += vertices_len as u32;
        }

        // Combine the individual meshes into a single mesh
        let mut final_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        final_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        final_mesh.set_indices(Some(Indices::U32(indices)));

        final_mesh
    }
}

fn compute_line_normal(start: Vec3, end: Vec3) -> Vec3 {
    let direction = end - start;
    let up = Vec3::new(0.0, 1.0, 0.0);
    let perp = if direction.cross(up).length() > 0.0001 {
        direction.cross(up).normalize()
    } else {
        direction.cross(Vec3::new(1.0, 0.0, 0.0)).normalize()
    };
    let normal = direction.cross(perp).normalize();
    normal
}


fn create_thick_line_mesh(lines: &[(Vec3, Vec3)], thickness: f32) -> Mesh {

    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();

    for (start, end) in lines {
        let normal = compute_line_normal(*start, *end);
        let offset = normal * (thickness / 2.0);
        let new_vertices = [
            *start - offset,
            *start + offset,
            *end - offset,
            *end + offset,
        ];
        let vertex_offset = vertices.len() as u32;

        vertices.extend_from_slice(&new_vertices);
        normals.extend_from_slice(&[normal; 4]);

        indices.extend_from_slice(&[
            vertex_offset + 0,
            vertex_offset + 1,
            vertex_offset + 2,
            vertex_offset + 1,
            vertex_offset + 2,
            vertex_offset + 3,
        ]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}
