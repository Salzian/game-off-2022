use bevy::prelude::*;
use bevy::{app::Plugin, sprite::MaterialMesh2dBundle};
use bevy_pathmesh::{PathMesh, PathmeshPlugin};

pub(crate) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PathmeshPlugin)
            .add_startup_system(load_assets)
            .add_system(setup_map);
    }
}

#[derive(Resource, Default)]
struct PathMeshResource {
    handle: Handle<PathMesh>,
    size: Vec2,
    loaded: bool,
}

fn load_assets(asset_server: Res<AssetServer>, mut commands: Commands) {
    let map_path_mesh_handle = asset_server.load("meshes/arena.polyanya.mesh");

    commands.insert_resource(PathMeshResource {
        handle: map_path_mesh_handle,
        size: Vec2::new(49.0, 49.0),
        ..default()
    });
}

fn setup_map(
    windows: Res<Windows>,
    path_mesh_assets: Res<Assets<PathMesh>>,
    mut commands: Commands,
    mut path_mesh_resource: ResMut<PathMeshResource>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<ColorMaterial>>,
) {
    let potential_map_path_mesh = path_mesh_assets.get(&path_mesh_resource.handle);

    match potential_map_path_mesh {
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

    let map_path_mesh = potential_map_path_mesh.unwrap();
    let map_mesh = map_path_mesh.to_mesh();

    let map_mesh_scaling_factor = (window.width() / path_mesh_resource.size.x)
        .min(window.height() / path_mesh_resource.size.y);
    let map_mesh_translation = Vec3::new(
        -path_mesh_resource.size.x / 2.0 * map_mesh_scaling_factor,
        -path_mesh_resource.size.y / 2.0 * map_mesh_scaling_factor,
        0.0,
    );

    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_assets.add(map_mesh).into(),
        material: material_assets.add(ColorMaterial::from(Color::GRAY)),
        transform: Transform::from_translation(map_mesh_translation)
            .with_scale(Vec3::splat(map_mesh_scaling_factor)),
        ..default()
    });
}
