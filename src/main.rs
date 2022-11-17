use bevy::prelude::App;
use bevy::DefaultPlugins;
use game_plugin::GamePlugin;

mod game_plugin;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
