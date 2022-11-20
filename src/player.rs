use bevy::prelude::*;
use std::ops::Add;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .add_system(control_player);
    }
}

#[derive(Bundle, Default)]
pub(crate) struct PlayerBundle {
    player: Player,
    sprite_bundle: SpriteBundle,
}

#[derive(Component, Default)]
pub(crate) struct Player;

fn setup_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation: PADDLE_INITIAL_POSITION,
                scale: PLAYER_INITIAL_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_INITIAL_COLOR,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

fn control_player(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut new_translation = Vec3::ZERO;

    if keyboard.pressed(KeyCode::Right) {
        // time.delta_seconds() is being used to make the player move at a
        // constant speed across different frame rates.
        new_translation = new_translation.add(PLAYER_SPEED * Vec3::X * time.delta_seconds());
    }

    if keyboard.pressed(KeyCode::Left) {
        new_translation = new_translation.add(PLAYER_SPEED * Vec3::NEG_X * time.delta_seconds());
    }

    if keyboard.pressed(KeyCode::Up) {
        new_translation = new_translation.add(PLAYER_SPEED * Vec3::Y * time.delta_seconds());
    }

    if keyboard.pressed(KeyCode::Down) {
        new_translation = new_translation.add(PLAYER_SPEED * Vec3::NEG_Y * time.delta_seconds());
    }

    player_transform.translation += new_translation;
}

const PADDLE_INITIAL_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0 /* Positions player on top */);
const PLAYER_INITIAL_SIZE: Vec3 = Vec3::new(50.0, 50.0, 0.0);
const PLAYER_INITIAL_COLOR: Color = Color::ORANGE_RED;

const PLAYER_SPEED: f32 = 500.0;
