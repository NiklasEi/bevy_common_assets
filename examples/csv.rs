use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_common_assets::csv::{CsvAssetPlugin, LoadedCsv};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CsvAssetPlugin::<TreePosition>::new(&["level.csv"]),
        ))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_level.run_if(in_state(AppState::Loading)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level = LevelHandle(asset_server.load("trees.level.csv"));
    commands.insert_resource(level);
    let tree = ImageHandle(asset_server.load("tree.png"));
    commands.insert_resource(tree);

    commands.spawn((Camera2d, Msaa::Off));
}

fn spawn_level(
    mut commands: Commands,
    level: Res<LevelHandle>,
    tree: Res<ImageHandle>,
    positions: Res<Assets<LoadedCsv<TreePosition>>>,
    mut state: ResMut<NextState<AppState>>,
) {
    if let Some(level) = positions.get(&level.0) {
        for position in level.rows.iter() {
            commands.spawn((
                Sprite::from_image(tree.0.clone()),
                Transform::from_translation(Vec3::new(position.x, position.y, position.z)),
            ));
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
