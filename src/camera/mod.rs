use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy::prelude::shape::Quad;
use bevy::render::camera::Viewport;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::{CursorGrabMode, WindowRef};
use bevy_mod_picking::DefaultPickingPlugins;
use crate::camera::lerping_camera::{camera_control, camera_rotation_system};
use crate::pickable_events::PickableEvent;

pub(crate) mod raycast_select;
pub(crate) mod lerping_camera;

pub const MOUSE_SENSITIVITY: f32 = 0.2;
pub const MIN_PITCH: f32 = -89.0;
pub const MAX_PITCH: f32 = 89.0;
pub const FORWARD_SENSITIVITY: f32 = 16.0;

#[derive(Resource, Default)]
pub struct ZoomableDraggableCamera {
    pub(crate) zoom: f32,
    pub(crate) cursor_position: Vec2,
    pub(crate) camera_position: Vec2,
    pub(crate) pitch: f32,
    pub(crate) yaw: f32,
    pub(crate) min_distance: f32,
    pub(crate) max_distance: f32,
    pub(crate) current_distance: f32,
    pub(crate) zoom_sensitivity: f32,
    pub(crate) initialized: bool,
    pub(crate) target_rotation: Option<Quat>,
    pub(crate) target_translation: Option<Vec3>
}


pub struct NnFeCameraPlugin;

impl Plugin for NnFeCameraPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPickingPlugins.build()
        )
            .add_startup_system(setup_camera)
            .add_system(camera_rotation_system)
            .add_system(camera_control)
            .add_event::<PickableEvent>();
    }
}

pub(crate) fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cam: ResMut<ZoomableDraggableCamera>
) {
    let initial_position = Vec3::new(50.0, 50.0, 100.0); // Replace with your desired initial position

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
            ..default()
        },
    ));

    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 9000.0,
    //         range: 100.,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(8.0, 16.0, 8.0),
    //     ..default()
    // });

}

