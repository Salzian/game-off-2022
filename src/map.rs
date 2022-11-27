use bevy::prelude::*;
use bevy::{app::Plugin, sprite::MaterialMesh2dBundle};
use bevy_asset_loader::prelude::*;
use bevy_pathmesh::{PathMesh, PathmeshPlugin};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MapState {
    LoadingAssets,
    Setup,
    Done,
}

pub(crate) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PathmeshPlugin)
            .add_loading_state(
                LoadingState::new(MapState::LoadingAssets)
                    .continue_to_state(MapState::Setup)
                    .with_collection::<MapAssets>(),
            )
            .add_state(MapState::LoadingAssets)
            .add_system_set(SystemSet::on_update(MapState::Setup).with_system(setup_map));
    }
}

#[derive(AssetCollection, Resource)]
struct MapAssets {
    #[asset(path = "meshes/arena.polyanya.mesh")]
    path_mesh_polyanya_map: Handle<PathMesh>,
}

fn setup_map(
    map_assets: Res<MapAssets>,
    path_mesh_assets: Res<Assets<PathMesh>>,
    windows: Res<Windows>,
    mut commands: Commands,
    mut map_state: ResMut<State<MapState>>,
    mut material_assets: ResMut<Assets<ColorMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
) {
    let window = windows.primary();

    let map_mesh = get_mesh_from_path_mesh_assets(&map_assets, path_mesh_assets);

    let map_mesh_scaling_factor = (window.width() / MAP_SIZE.x).min(window.height() / MAP_SIZE.y);

    let map_mesh_translation = Vec3::new(
        -MAP_SIZE.x / 2.0 * map_mesh_scaling_factor,
        -MAP_SIZE.y / 2.0 * map_mesh_scaling_factor,
        0.0,
    );

    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_assets.add(map_mesh).into(),
        material: material_assets.add(ColorMaterial::from(Color::GRAY)),
        transform: Transform::from_translation(map_mesh_translation)
            .with_scale(Vec3::splat(map_mesh_scaling_factor)),
        ..default()
    });

    // Finally, set the map state to done, as everything is set up now.
    map_state.set(MapState::Done).unwrap();
}

fn get_mesh_from_path_mesh_assets(
    map_assets: &Res<MapAssets>,
    path_mesh_assets: Res<Assets<PathMesh>>,
) -> Mesh {
    let map_path_mesh = path_mesh_assets
        .get(&map_assets.path_mesh_polyanya_map)
        .unwrap();

    map_path_mesh.to_mesh()
}

const MAP_SIZE: Vec2 = Vec2::new(49.0, 49.0);
