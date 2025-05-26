use bevy::prelude::*;

use crate::helpers::rect;

use super::hitbox::Hitbox;

pub const TRAP_PNG_RESOLUTION: Vec2 = vec2(341.0, 165.0);
pub const TRAP_DIMENSIONS: Vec2 = vec2(
    TRAP_PNG_RESOLUTION.x / 4.0, //
    TRAP_PNG_RESOLUTION.y / 4.0,
);

#[derive(Debug, Clone, Copy, Component)]
pub struct Trap;
impl Trap {
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        let mut sprite = Sprite::from_image(asset_server.load("trap.png"));
        sprite.custom_size = Some(TRAP_DIMENSIONS);

        commands.spawn((
            Trap, //
            sprite,
        ));
    }
}
