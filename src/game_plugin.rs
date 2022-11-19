use crate::player::{Player, PlayerPlugin};
use bevy::app::Plugin;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_startup_system(setup_camera)
            .add_startup_system(setup_scene)
            .add_system(camera_follow);
    }
}

// TODO Evaluate scene system
fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let grid = asset_server.load("textures/duck.png");

    commands.spawn(SpriteBundle {
        texture: grid,
        ..default()
    });
}

fn camera_follow(
    query_paddle_transform: Query<&mut Transform, With<Player>>,
    mut query_camera_transform: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
) {
    let paddle_transform = query_paddle_transform.single();
    let mut camera_transform = query_camera_transform.single_mut();

    camera_transform.translation = camera_transform
        .translation
        .lerp(paddle_transform.translation, time.delta_seconds() * 5.0);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
