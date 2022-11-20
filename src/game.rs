use crate::map::MapPlugin;
use crate::player::{Player, PlayerPlugin};
use bevy::app::Plugin;
use bevy::prelude::*;

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MapPlugin)
            .add_plugin(PlayerPlugin)
            .add_startup_system(setup_camera)
            .add_system(move_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn move_camera(
    player_transform_query: Query<&mut Transform, With<Player>>,
    mut camera_transform_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
) {
    let player_transform = player_transform_query.single();
    let mut camera_transform = camera_transform_query.single_mut();

    // time.delta_seconds() is being used to make the camera move at a constant
    // speed across different frame rates.
    camera_transform.translation = camera_transform
        .translation
        .lerp(player_transform.translation, time.delta_seconds() * 5.0);
}
