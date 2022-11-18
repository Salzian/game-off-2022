use crate::player_plugin::{Player, PlayerPlugin};
use bevy::app::Plugin;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_startup_system(setup_camera)
            .add_startup_system(setup_scene)
            .add_system(move_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// TODO Evaluate scene system
fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let duck_texture = asset_server.load("textures/duck.png");

    commands.spawn(SpriteBundle {
        texture: duck_texture,
        ..default()
    });
}

fn move_camera(
    player_transform_query: Query<&mut Transform, With<Player>>,
    mut camera_transform_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
) {
    let player_transform = player_transform_query.single();
    let mut camera_transform = camera_transform_query.single_mut();

    camera_transform.translation = camera_transform
        .translation
        .lerp(player_transform.translation, time.delta_seconds() * 5.0);
}
