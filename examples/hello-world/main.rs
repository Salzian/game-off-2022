use bevy::{
    prelude::{
        default, App, Camera2dBundle, Color, Commands, Component, Input, KeyCode, Plugin, Query,
        Res, Transform, Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_startup_system(setup_paddle)
            .add_system(control_paddle);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_paddle(mut commands: Commands) {
    commands.spawn().insert(Paddle).insert_bundle(SpriteBundle {
        transform: Transform {
            translation: PADDLE_INITIAL_POSITION,
            scale: PADDLE_INITIAL_SIZE,
            ..default()
        },
        sprite: Sprite {
            color: PADDLE_INITIAL_COLOR,
            ..default()
        },
        ..default()
    });
}

fn control_paddle(mut query: Query<&mut Transform, With<Paddle>>, keyboard: Res<Input<KeyCode>>) {
    let mut paddle_transform = query.single_mut();

    let direction = if keyboard.pressed(KeyCode::Right) {
        1.0
    } else if keyboard.pressed(KeyCode::Left) {
        -1.0
    } else {
        0.0
    };

    paddle_transform.translation.x += PADDLE_TRANSLATION_X_INCREMENT * direction;
}

const PADDLE_INITIAL_COLOR: Color = Color::ORANGE_RED;
const PADDLE_INITIAL_POSITION: Vec3 = Vec3::new(10.0, 10.0, 0.0);
const PADDLE_INITIAL_SIZE: Vec3 = Vec3::new(250.0, 10.0, 0.0);
const PADDLE_TRANSLATION_X_INCREMENT: f32 = 5.0;

#[derive(Component)]
struct Paddle;
