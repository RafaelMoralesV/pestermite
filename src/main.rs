use bevy::{
    prelude::*,
    window::{WindowLevel, WindowResolution},
};

fn main() {
    let window = Window {
        transparent: true,
        decorations: false,
        window_level: WindowLevel::AlwaysOnTop,
        resolution: WindowResolution::new(400.0, 250.0),
        resizable: false,
        ..default()
    };

    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }))
        .run();
}
