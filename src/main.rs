mod components;
mod helpers;
mod input_handlers;

use crate::{components::Grid, input_handlers::move_camera_with_arrow_keys};

use bevy::{
    app::PluginGroupBuilder,
    color::palettes::css::LIGHT_GRAY,
    input::{common_conditions::input_pressed, keyboard::keyboard_input_system},
    prelude::*,
    window::WindowResolution,
};
use components::{Player, trap::Trap};

fn custom_default_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(ImagePlugin::default())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("jammi"),
                resolution: WindowResolution::new(800.0, 800.0),
                ..default()
            }),
            ..default()
        })
        .build()
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            is_active: true,
            clear_color: ClearColorConfig::Custom(LIGHT_GRAY.into()),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn main() {
    App::new()
        .add_plugins(custom_default_plugins())
        .add_systems(
            Startup,
            (
                spawn_camera,
                Grid::spawn, //
                Player::spawn,
                Trap::spawn,
            ),
        )
        .add_systems(
            Update,
            (
                keyboard_input_system,
                Player::move_with_arrow_keys.after(keyboard_input_system),
            ),
        )
        .run();
}
