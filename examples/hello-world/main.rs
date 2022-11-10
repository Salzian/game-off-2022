use bevy::{
    prelude::{
        default, shape, App, Assets, Camera2dBundle, Color, Commands, Mesh, Plugin, ResMut,
        Transform, Vec3,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
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
        app.add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let paddle_color: Color = Color::ORANGE_RED;
    let paddle_translation: Vec3 = Vec3::new(10.0, 10.0, 0.0);
    let paddle_size: Vec3 = Vec3::new(250.0, 10.0, 0.0);

    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        material: materials.add(ColorMaterial::from(paddle_color)),
        transform: Transform {
            translation: paddle_translation,
            scale: paddle_size,
            ..default()
        },
        ..default()
    });
}
