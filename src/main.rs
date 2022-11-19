use bevy::prelude::*;
use game_plugin::GamePlugin;

mod game_plugin;
mod player_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Game Off 2022".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                resizable: false,
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .add_plugin(GamePlugin)
        .run();
}

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
