use crate::player_plugin::{Player, PlayerPlugin};
use bevy::prelude::*;
use bevy::{app::Plugin, sprite::MaterialMesh2dBundle};
use bevy_pathmesh::{PathMesh, PathmeshPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(PathmeshPlugin)
            .add_startup_system(load_assets)
            .add_startup_system(setup_camera)
            .add_system(setup_arena)
            .add_system(move_camera);
    }
}

#[derive(Resource, Default)]
struct PathMeshResource {
    handle: Handle<PathMesh>,
    size: Vec2,
    loaded: bool,
}

fn load_assets(asset_server: Res<AssetServer>, mut commands: Commands) {
    let arena_path_mesh_handle = asset_server.load("arena.polyanya.mesh"); // TODO: Move into meshes directory

    commands.insert_resource(PathMeshResource {
        handle: arena_path_mesh_handle,
        size: Vec2::new(49.0, 49.0),
        ..default()
    });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_arena(
    windows: Res<Windows>,
    path_mesh_assets: Res<Assets<PathMesh>>,
    mut commands: Commands,
    mut path_mesh_resource: ResMut<PathMeshResource>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<ColorMaterial>>,
) {
    let potential_arena_path_mesh = path_mesh_assets.get(&path_mesh_resource.handle);

    match potential_arena_path_mesh {
        // If the resource hasn't been loaded yet:
        None => {
            return;
        }
        // If the resource has been loaded before:
        Some(_) => {
            // Was the resource already marked as loaded?
            if path_mesh_resource.loaded {
                return;
            }
            // Has the resource just been freshly loaded?
            else {
                path_mesh_resource.loaded = true; // Mark it as loaded
            }
        }
    }

    let window = windows.primary();

    let arena_path_mesh = potential_arena_path_mesh.unwrap();
    let arena_mesh = arena_path_mesh.to_mesh();

    let arena_mesh_scaling_factor = (window.width() / path_mesh_resource.size.x)
        .min(window.height() / path_mesh_resource.size.y);
    let arena_mesh_translation = Vec3::new(
        -path_mesh_resource.size.x / 2.0 * arena_mesh_scaling_factor,
        -path_mesh_resource.size.y / 2.0 * arena_mesh_scaling_factor,
        0.0,
    );

    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_assets.add(arena_mesh).into(),
        material: material_assets.add(ColorMaterial::from(Color::GRAY)),
        transform: Transform::from_translation(arena_mesh_translation)
            .with_scale(Vec3::splat(arena_mesh_scaling_factor)),
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
