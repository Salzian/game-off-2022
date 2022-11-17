use bevy::prelude::*;
use std::ops::Add;

#[derive(Component, Default)]
pub(crate) struct Player;

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

fn setup_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        sprite_bundle: SpriteBundle {
            transform: Transform {
                scale: PADDLE_INITIAL_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_INITIAL_COLOR,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

fn control_player(mut query: Query<&mut Transform, With<Player>>, keyboard: Res<Input<KeyCode>>) {
    let mut paddle_transform = query.single_mut();

    let mut new_translation = Vec3::ZERO;

    if keyboard.pressed(KeyCode::Right) {
        new_translation = new_translation.add(PADDLE_TRANSLATION_X_INCREMENT)
    }

    if keyboard.pressed(KeyCode::Left) {
        new_translation = new_translation.add(PADDLE_TRANSLATION_X_INCREMENT * Vec3::NEG_X)
    }

    if keyboard.pressed(KeyCode::Up) {
        new_translation = new_translation.add(PADDLE_TRANSLATION_Y_INCREMENT)
    }

    if keyboard.pressed(KeyCode::Down) {
        new_translation = new_translation.add(PADDLE_TRANSLATION_Y_INCREMENT * Vec3::NEG_Y);
    }

    paddle_transform.translation += new_translation;
}

const PADDLE_INITIAL_COLOR: Color = Color::ORANGE_RED;
const PADDLE_INITIAL_SIZE: Vec3 = Vec3::new(50.0, 50.0, 0.0);
const PADDLE_TRANSLATION_X_INCREMENT: Vec3 = Vec3::new(5.0, 0.0, 0.0);
const PADDLE_TRANSLATION_Y_INCREMENT: Vec3 = Vec3::new(0.0, 5.0, 0.0);
