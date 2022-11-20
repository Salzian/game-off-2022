use bevy::prelude::*;
use debug::DebugPlugin;
use game::GamePlugin;

mod debug;
mod game;
mod map;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Game Off 2022".to_string(),
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .add_plugin(DebugPlugin)
        .add_plugin(GamePlugin)
        .run();
}
