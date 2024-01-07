use bevy::asset::RecursiveDependencyLoadState;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_common_assets::csv::{CsvAssetPlugin, LoadedCsv};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CsvAssetPlugin::<TreePosition>::new(&["level.csv"]),
        ))
        .insert_resource(Msaa::Off)
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_level.run_if(in_state(AppState::Loading)))
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level = LevelHandle(asset_server.load("trees.level.csv"));
    commands.insert_resource(level);
    let tree = ImageHandle(asset_server.load("tree.png"));
    commands.insert_resource(tree);

    commands.spawn(Camera2dBundle::default());
}

fn spawn_level(
    mut commands: Commands,
    level: Res<LevelHandle>,
    asset_server: Res<AssetServer>,
    tree: Res<ImageHandle>,
    mut positios: ResMut<Assets<TreePosition>>,
    mut state: ResMut<NextState<AppState>>,
) {
    if asset_server.get_recursive_dependency_load_state(&level.0)
        == Some(RecursiveDependencyLoadState::Loaded)
    {
        for (_, position) in positios.iter() {
            commands.spawn(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.x, position.y, position.z,
                )),
                texture: tree.0.clone(),
                ..default()
            });
        }

        state.set(AppState::Level);
    }
}

#[derive(serde::Deserialize, Asset, TypePath, Debug)]
struct TreePosition {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Level,
}

#[derive(Resource)]
struct ImageHandle(Handle<Image>);

#[derive(Resource)]
struct LevelHandle(Handle<LoadedCsv<TreePosition>>);
