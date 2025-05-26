use bevy::{
    app::PluginGroupBuilder, color::palettes::css::LIGHT_GRAY, prelude::*, window::WindowResolution,
};

pub fn custom_default_plugins() -> PluginGroupBuilder {
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

pub fn spawn_camera(mut commands: Commands) {
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
