use std::f32::consts::{PI, SQRT_2};

use bevy::prelude::*;

use crate::components::Velocity;

pub const PLAYER_PNG_RESOLUTION: Vec2 = vec2(133.0, 341.0);
pub const PLAYER_DIMENSIONS: Vec2 = vec2(
    PLAYER_PNG_RESOLUTION.x / 4.0, //
    PLAYER_PNG_RESOLUTION.y / 4.0,
);
pub const PLAYER_INITIAL_ROTATION: Quat = Quat::from_xyzw(0.0, 0.0, SQRT_2 / 2.0, SQRT_2 / 2.0);

#[derive(Debug, Clone, Copy, Component)]
pub struct Player;
impl Player {
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        let mut sprite = Sprite::from_image(asset_server.load("player.png"));
        sprite.custom_size = Some(PLAYER_DIMENSIONS);

        let mut transform = Transform::from_xyz(0.0, 0.0, 1.0);
        transform.rotation = PLAYER_INITIAL_ROTATION;

        commands.spawn((Player, Velocity::ZERO, sprite, transform));
    }
}
