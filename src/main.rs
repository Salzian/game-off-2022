#![allow(clippy::too_many_arguments)] // Welcome to bevy

use bevy::prelude::*;
use game::GamePlugin;

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
        .add_plugin(GamePlugin)
        .run();
}
