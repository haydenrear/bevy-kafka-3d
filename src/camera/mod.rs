use bevy::core_pipeline::clear_color::ClearColorConfig;
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
    pub(crate) pitch: f32,
    pub(crate) yaw: f32,
    pub(crate) is_dragging: bool,
    pub(crate) min_distance: f32,
    pub(crate) max_distance: f32,
    pub(crate) current_distance: f32,
    pub(crate) zoom_sensitivity: f32,
    pub(crate) initialized: bool
}

pub(crate) fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cam: ResMut<ZoomableDraggableCamera>
) {
    let initial_position = Vec3::new(-2.0, 2.5, -1000.0); // Replace with your desired initial position

    let mut initial = Transform::from_translation(initial_position)
        .looking_at(Vec3::ZERO, Vec3::Y);

    let forward = initial.compute_matrix().z_axis.normalize();
    let pitch = (-forward.y).asin().to_degrees();
    let yaw = forward.x.atan2(forward.z).to_degrees();

    cam.pitch = pitch;
    cam.yaw = yaw;

    commands.spawn((
        Camera3dBundle {
            transform: initial,
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::WHITE),
                ..default()
            },
            ..default()
        },
        PickingCameraBundle::default()
    ));


}

pub const MOUSE_SENSITIVITY: f32 = 0.5;
pub const MIN_PITCH: f32 = -89.0;
pub const MAX_PITCH: f32 = 89.0;

pub(crate) fn camera_control(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut camera_drag_data: ResMut<ZoomableDraggableCamera>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut ev_mousse: EventReader<MouseMotion>,
) {


    for event in ev_mousse.iter() {
        if mouse_button_input.pressed(MouseButton::Left) {
            camera_drag_data.yaw -= event.delta.x * MOUSE_SENSITIVITY;
            camera_drag_data.pitch += event.delta.y * MOUSE_SENSITIVITY;

            // Clamp pitch to avoid gimbal lock
            // camera_drag_data.pitch = camera_drag_data.pitch.clamp(MIN_PITCH, MAX_PITCH);

            // Calculate new camera rotation quaternion
            let yaw_quat = Quat::from_axis_angle(Vec3::Y, camera_drag_data.yaw.to_radians());
            let pitch_quat = Quat::from_axis_angle(Vec3::X, camera_drag_data.pitch.to_radians());
            let rotation = yaw_quat * pitch_quat;

            // Apply the new rotation to the camera
            for mut transform in camera_query.iter_mut() {
                transform.1.rotation = rotation;
            }
        }
    }

    for event in mouse_wheel.iter() {

        camera_drag_data.current_distance -= event.y * camera_drag_data.zoom_sensitivity;

        // Clamp current distance to the range [min_distance, max_distance]
        camera_drag_data.current_distance = camera_drag_data.current_distance.clamp(camera_drag_data.min_distance, camera_drag_data.max_distance);

        if let Some((_, mut transform)) = camera_query.iter_mut().next() {
            transform.translation.z = camera_drag_data.current_distance;
        }
    }

}

