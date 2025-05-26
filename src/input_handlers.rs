use crate::components::Player;

use std::{f32::consts::PI, ops::DerefMut};

use bevy::prelude::*;

pub fn move_camera_with_arrow_keys(
    input: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    const CAMERA_SPEED: f32 = 1.0;
    if input.pressed(KeyCode::ArrowUp) {
        camera.deref_mut().translation += Vec3::Y * CAMERA_SPEED;
    }
    if input.pressed(KeyCode::ArrowDown) {
        camera.deref_mut().translation += -Vec3::Y * CAMERA_SPEED;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        camera.deref_mut().translation += -Vec3::X * CAMERA_SPEED;
    }
    if input.pressed(KeyCode::ArrowRight) {
        camera.deref_mut().translation += Vec3::X * CAMERA_SPEED;
    }
}

impl Player {
    pub const SPEED: f32 = 9.0;
    pub fn move_with_arrow_keys(
        input: Res<ButtonInput<KeyCode>>,
        mut player: Single<&mut Transform, With<Player>>,
    ) {
        let mut key_pressed = false;

        let mut delta = Transform::from_xyz(0.0, 0.0, 0.0);
        if input.pressed(KeyCode::ArrowUp) {
            delta.translation += Vec3::Y * Self::SPEED;
            delta.rotate(Quat::from_rotation_z(PI));
            key_pressed = true;
        }
        if input.pressed(KeyCode::ArrowDown) {
            delta.translation += -Vec3::Y * Self::SPEED;
            key_pressed = true;
            delta.rotation = Quat::from_rotation_z(0.0);
        }
        if input.pressed(KeyCode::ArrowLeft) {
            delta.translation += -Vec3::X * Self::SPEED;
            delta.rotation = Quat::from_rotation_z(const { 3.0 * PI / 2.0 });
            key_pressed = true;
        }
        if input.pressed(KeyCode::ArrowRight) {
            delta.translation += Vec3::X * Self::SPEED;
            delta.rotation = Quat::from_rotation_z(const { PI / 2.0 });
            key_pressed = true;
        }

        player.translation += delta.translation;
        if key_pressed {
            player.rotation = delta.rotation;
        }
    }
}
