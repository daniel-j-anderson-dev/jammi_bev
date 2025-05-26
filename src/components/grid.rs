use bevy::prelude::*;

pub const GRID_LINE_COUNT: usize = 10;
pub const GRID_WIDTH: f32 = 500.0;
pub const GRID_HEIGHT: f32 = 500.0;
pub const GRID_DIMENSIONS: Vec2 = vec2(CELL_WIDTH, CELL_HEIGHT);
pub const CELL_WIDTH: f32 = GRID_WIDTH / GRID_LINE_COUNT as f32;
pub const CELL_HEIGHT: f32 = GRID_HEIGHT / GRID_LINE_COUNT as f32;
pub const CELL_DIMENSIONS: Vec2 = vec2(CELL_WIDTH, CELL_HEIGHT);

#[derive(Debug, Clone, Copy, Component)]
pub struct Grid;
impl Grid {
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((Grid, Sprite::from_image(asset_server.load("grid.png"))));
    }
}
