mod components;
mod helpers;
mod input_handlers;
mod setup;

use crate::{
    components::{Grid, Player, Trap},
    setup::{custom_default_plugins, spawn_camera},
};

use bevy::{input::keyboard::keyboard_input_system, prelude::*};

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
