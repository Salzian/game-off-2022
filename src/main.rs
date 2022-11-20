use bevy::prelude::*;
use game_plugin::GamePlugin;

mod game_plugin;
mod player_plugin;

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
