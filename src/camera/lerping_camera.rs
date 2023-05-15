use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use crate::camera::{MOUSE_SENSITIVITY, ZoomableDraggableCamera};

pub(crate) fn camera_rotation_system(
    time: Res<Time>,
    mut camera_drag_data: ResMut<ZoomableDraggableCamera>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>
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

