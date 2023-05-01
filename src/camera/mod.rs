use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::prelude::shape::Quad;
use bevy::render::camera::Viewport;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::{CursorGrabMode, WindowRef};
use bevy_mod_picking::PickingCameraBundle;

#[derive(Resource, Default)]
pub struct ZoomableDraggableCamera {
    pub(crate) zoom: f32,
    pub(crate) cursor_position: Vec2,
    pub(crate) camera_position: Vec2,
    pub(crate) is_dragging: bool
}

pub(crate) fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), PickingCameraBundle::default()));
    commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Quad::new(Vec2::new(10000.0, 10000.0)))).into(),
            transform: Transform::default(),
            material: materials.add(ColorMaterial::from(Color::BEIGE)),
            ..default()
        });
}

pub(crate) fn camera_control(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut windows: Query<&mut Window>,
    mut camera_drag_data: ResMut<ZoomableDraggableCamera>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: Res<AssetServer>,
) {


    let cursor_position = if let Some(event) = cursor_moved.iter().next() {
        event.position
    } else {
        return;
    };

    let window = windows.get_single().unwrap();
    let size = Vec2::new(window.width() as f32, window.height() as f32);

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some((_, mut transform)) = camera_query.iter_mut().next() {
            let camera_translation = transform.translation;
            camera_drag_data.cursor_position = cursor_position;
            camera_drag_data.camera_position.x = camera_translation.x;
            camera_drag_data.camera_position.y = camera_translation.y;
        }
    } else if mouse_button_input.pressed(MouseButton::Left) {
        if let Some((_, mut transform)) = camera_query.iter_mut().next() {
            let delta = cursor_position - camera_drag_data.cursor_position;
            transform.translation.x = camera_drag_data.camera_position.x - delta.x;
            transform.translation.y = camera_drag_data.camera_position.y - delta.y;
        }
    }

    for mouse_wheel_event in mouse_wheel.iter() {
        if let Some((_, mut transform)) = camera_query.iter_mut().next() {
            let y_val = mouse_wheel_event.y;

            let val = 0.1;
            if transform.scale.z + val < 0.0 || transform.scale.z - val < 0.0 {
                return;
            }
            if y_val > 0.0 {
                transform.scale += Vec3::new(val, val, val);
            } else if y_val < 0.0 {
                transform.scale -= Vec3::new(val, val, val);
            }
        }
    }

}

