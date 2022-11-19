use crate::player_plugin::{Player, PlayerPlugin};
use bevy::prelude::*;
use bevy::{app::Plugin, sprite::MaterialMesh2dBundle};
use bevy_pathmesh::{PathMesh, PathmeshPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(PathmeshPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
            //.add_startup_system(setup_scene)
            .add_startup_system(setup_camera)
            .add_system(await_loading_assets)
            .add_system(move_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
struct PathMeshResource {
    path_mesh: Handle<PathMesh>,
    loaded: bool,
}

fn load_assets(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle = asset_server.load("arena-merged.polyanya.mesh");
    commands.insert_resource(PathMeshResource {
        path_mesh: handle,
        loaded: false,
    });
}

// TODO Evaluate scene system
fn setup_scene(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    path_mesh_assets: Res<Assets<PathMesh>>,
    mut material_assets: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    // path_mesh_resources: Res<PathMeshResource>,
) {
    // commands.insert_resource(ClearColor(Color::BLACK));

    // commands.insert_resource(PathMeshResource(path_mesh_handle));
    // let duck_texture = asset_server.load("textures/duck.png");

    // commands.spawn(SpriteBundle {
    //     texture: duck_texture,
    //     ..default()
    // });

    // let handle = &path_mesh_resources.path_mesh;

    let handle = asset_server.load("arena-merged.polyanya.mesh");
    let arena_mesh = path_mesh_assets.get(&handle).unwrap();

    let mesh_size = Vec2::new(49.0, 49.0);
    let window = windows.primary();
    let factor = (window.width() / mesh_size.x).min(window.height() / mesh_size.y);

    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_assets.add(arena_mesh.to_mesh()).into(),
        material: material_assets.add(ColorMaterial::from(Color::GRAY)),
        transform: Transform::from_translation(Vec3::new(
            -mesh_size.x / 2.0 * factor,
            -mesh_size.y / 2.0 * factor,
            0.0,
        ))
        .with_scale(Vec3::splat(factor)),
        ..default()
    });
}

fn await_loading_assets(
    path_mesh_assets: Res<Assets<PathMesh>>,
    mut path_mesh_resource: ResMut<PathMeshResource>,
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    let arena_mesh = path_mesh_assets.get(&path_mesh_resource.path_mesh);

    match arena_mesh {
        None => {
            return;
        }
        Some(_) => {
            path_mesh_resource.loaded = true;
        }
    }

    let arena_mesh = arena_mesh.unwrap();

    let mesh_size = Vec2::new(49.0, 49.0);
    let window = windows.primary();
    let factor = (window.width() / mesh_size.x).min(window.height() / mesh_size.y);

    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_assets.add(arena_mesh.to_mesh()).into(),
        material: material_assets.add(ColorMaterial::from(Color::GRAY)),
        transform: Transform::from_translation(Vec3::new(
            -mesh_size.x / 2.0 * factor,
            -mesh_size.y / 2.0 * factor,
            0.0,
        ))
        .with_scale(Vec3::splat(factor)),
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
