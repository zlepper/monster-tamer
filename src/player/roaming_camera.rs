use crate::player::Player;
use crate::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use std::f32::consts::PI;

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq)]
pub enum CameraMovement {
    Rotate,
}

const CAMERA_PAN_RATE: f32 = 0.5;
pub const DEFAULT_CAMERA_DISTANCE: f32 = 10.0;
pub const DEFAULT_CAMERA_VECTOR: Vec3 = Vec3::new(0.0, 3.0, DEFAULT_CAMERA_DISTANCE);

pub fn has_window_focus(windows: Query<&Window, With<PrimaryWindow>>) -> bool {
    let window = windows.get_single();

    if let Ok(window) = window {

        window.focused && window.cursor.grab_mode == CursorGrabMode::Locked
    } else {
        false
    }
}

pub fn rotate_player(
    mut camera_query: Query<(&ActionState<CameraMovement>, &Parent), With<Camera3d>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Camera3d>)>,
) {
    for (action_state, parent) in camera_query.iter_mut() {
        if let Ok(mut player_transform) = player_query.get_mut(parent.get()) {
            if let Some(camera_pan_vector) = action_state.axis_pair(CameraMovement::Rotate) {
                if camera_pan_vector.x() != 0.0 {
                    player_transform
                        .rotate_local_y(-(camera_pan_vector.x() * CAMERA_PAN_RATE).to_radians());
                }
            }
        }
    }
}

pub fn pan_camera(
    mut camera_query: Query<(&mut Transform, &mut KinematicCharacterController, &ActionState<CameraMovement>), With<Camera3d>>,
) {
    for (mut camera_transform, mut character_controller, action_state) in camera_query.iter_mut() {
        if let Some(camera_pan_vector) = action_state.axis_pair(CameraMovement::Rotate) {
            if camera_pan_vector.y() != 0.0 {
                let mut updated = *camera_transform;

                updated.rotate_around(
                    Vec3 {
                        z: 0.0,
                        x: 0.0,
                        ..DEFAULT_CAMERA_VECTOR
                    },
                    Quat::from_rotation_x((-camera_pan_vector.y() * CAMERA_PAN_RATE).to_radians()),
                );

                let updated_angle = Quat::IDENTITY.angle_between(updated.rotation.normalize());

                if updated_angle < PI * 0.45 {

                    let forward = updated.forward();

                    let scale = if forward.y == 0.0 {
                        1.0
                    } else if forward.y > 0.0 {
                        remap(updated_angle, 0.0, PI * 0.45, 1.0, 0.5)
                    } else {
                        remap(updated_angle, 0.0, PI * 0.45, 1.0, 2.0)
                    };

                    let mut new_translation = forward * scale * DEFAULT_CAMERA_DISTANCE * -1.0;
                    new_translation.y += DEFAULT_CAMERA_VECTOR.y;

                    updated.translation = new_translation;
                    character_controller.translation = Some(updated.translation - camera_transform.translation);
                    camera_transform.rotation = updated.rotation;
                }
            }
        }
    }
}




fn remap(value: f32, value_min: f32, value_max: f32, result_min: f32, result_max: f32) -> f32 {
    (value - value_min) / (value_max - value_min) * (result_max - result_min) + result_min
}


pub fn grab_mouse(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.get_single_mut();

    if let Ok(mut window) = window {
        if window.cursor_position().is_some() {
            if mouse.just_pressed(MouseButton::Left) {
                window.cursor.visible = false;
                window.cursor.grab_mode = CursorGrabMode::Locked;
            }

            if key.just_pressed(KeyCode::Escape) {
                window.cursor.visible = true;
                window.cursor.grab_mode = CursorGrabMode::None;
            }
        }
    }
}
