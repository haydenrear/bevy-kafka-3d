use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::math::Vec4Swizzles;
use crate::camera::{FORWARD_SENSITIVITY, MOUSE_SENSITIVITY, ZoomableDraggableCamera};
use crate::camera::raycast_select::BevyPickingState;

pub(crate) fn camera_rotation_system(
    time: Res<Time>,
    mut camera_drag_data: ResMut<ZoomableDraggableCamera>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
) {

    for input in mouse_button_input_events.iter() {
        if input.state == ButtonState::Released {
            camera_drag_data.target_rotation = None;
        }
    }
    for (cam, mut transform) in camera_query.iter_mut() {
        // Use slerp to gradually interpolate from the current rotation to the target rotation
        camera_drag_data.target_rotation.map(|t| {
            if mouse_button_input.pressed(MouseButton::Left) {
                transform.rotation = transform.rotation.slerp(
                    t,
                    time.delta_seconds() * 20.0
                );
            }
        });
        camera_drag_data.target_translation.map(|translation| {
            transform.translation = transform.translation.lerp(
                translation,
                time.delta_seconds() * 3.0
            );
        });

    }
}

pub(crate) fn camera_control(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    pick_state: Res<BevyPickingState>,
    mut camera_drag_data: ResMut<ZoomableDraggableCamera>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut ev_mousse: EventReader<MouseMotion>,
    mut keyboard_events: EventReader<KeyboardInput>,
) {

    if pick_state.picked_ui_flag {
        return;
    }

    for key in keyboard_events.iter() {
        let (_cam, mut transform) = camera_query.single_mut();
        let cam_matrix = transform.compute_matrix();
        let forward = -cam_matrix.z_axis.xyz().normalize();
        let right = cam_matrix.x_axis.xyz().normalize();
        let mut movement = Vec3::ZERO;
        if let Some(KeyCode::W) = key.key_code {
            movement += forward;
        } else if let Some(KeyCode::S) = key.key_code {
            movement -= forward;
        } else if let Some(KeyCode::D) = key.key_code {
            movement += right;
        } else if let Some(KeyCode::A) = key.key_code {
            movement -= right;
        }
        if movement.length() != 0.0 {
            movement = movement.normalize() * FORWARD_SENSITIVITY;
            camera_drag_data.target_translation = Some(transform.translation + movement);
        }
    }

    for event in ev_mousse.iter() {
        if mouse_button_input.pressed(MouseButton::Left) {
            camera_drag_data.yaw -= event.delta.x * MOUSE_SENSITIVITY;
            camera_drag_data.pitch -= event.delta.y * MOUSE_SENSITIVITY;

            let yaw_quat = Quat::from_axis_angle(Vec3::Y, camera_drag_data.yaw.to_radians());
            let pitch_quat = Quat::from_axis_angle(Vec3::X, camera_drag_data.pitch.to_radians());
            let rotation = yaw_quat * pitch_quat;

            camera_drag_data.target_rotation = Some(rotation);
        }
    }

}

