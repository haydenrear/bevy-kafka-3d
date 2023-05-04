use std::path::Path;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::PickableBundle;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::prelude::{Fill, Stroke};
use bevy_prototype_lyon::shapes;
use crate::lines::{create_3d_line, LineList, LineMaterial};

#[derive(Resource)]
struct GraphData {
    points: Vec<Vec3>,
}

// impl Default for GraphData {
//     fn default() -> Self {
//         Self {
//         }
//     }
// }

pub(crate) fn setup_graph(mut commands: Commands,
                          mut meshes: ResMut<Assets<Mesh>>,
                          mut materials: ResMut<Assets<LineMaterial>>,
                          asset_server: Res<AssetServer>
) {
    let grid_size = 1000.0;
    let grid_count = 10;


    // X plane gridlines (parallel to the YZ plane)
    create_gridlines(&mut commands,&mut meshes, &mut materials,'x', grid_size, grid_count, Color::rgba(1.0, 0.0, 0.0, 0.25));
    create_gridlines(&mut commands,&mut meshes, &mut materials, 'y', grid_size, grid_count, Color::rgba(0.0, 1.0, 0.0, 0.25));
    create_gridlines(&mut commands,&mut meshes, &mut materials,'z', grid_size, grid_count, Color::rgba(0.0, 0.0, 1.0, 0.25));
}

fn create_gridlines(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<LineMaterial>>,
    axis: char,
    size: f32,
    count: usize,
    color: Color
)  {
    let spacing = size / count as f32;

    for i in 0..=count {
        let offset = i as f32 * spacing - size / 2.0;
        info!("Adding axis");
        let (start, end) = match axis {
            'x' => (Vec3::new(offset, -size / 2.0, 0.0), Vec3::new(offset, size / 2.0, 1.0)),
            'y' => (Vec3::new(-size / 2.0, offset, 0.0), Vec3::new(size / 2.0, offset, 1.0)),
            'z' => (Vec3::new(-size / 2.0, 0.0, offset), Vec3::new(size / 2.0, 0.0, offset)),
            _ => panic!("Invalid axis"),
        };

        create_line(commands, start, end, meshes, materials);
    }

}


// fn draw_graph(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<LineMaterial>>,
//     graph_data: Res<GraphData>,
// ) {
//     let mut prev: Option<&Vec3> = None;
//     for point in &graph_data.points {
//         if prev.is_some() {
//             create_line(&mut commands, prev.unwrap(), &point);
//             let line_material = materials.add(Color::BLACK.into());
//             commands.spawn(PbrBundle {
//                 mesh: meshes.add(mesh),
//                 material: line_material,
//                 transform,
//                 ..Default::default()
//             });
//         }
//         prev = Some(point);
//     }
// }


fn create_line(
    commands: &mut Commands,
    start: Vec3,
    end: Vec3,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<LineMaterial>>,
)  {
    let vec3 = end - start;
    let distance = (vec3).length();
    let midpoint = (start + end) / 2.0;
    let rotation = Quat::from_rotation_arc(Vec3::Y, vec3);

    let transform = Transform {
        translation: midpoint,
        rotation,
        scale: Vec3::ONE,
    };

    let line_mesh = create_3d_line(LineList {
        lines: vec![
            (start, end)
        ],
    });

    commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(line_mesh.0),
                material: materials.add(line_mesh.1),
                transform,
                ..default()
            }
        ));

}
