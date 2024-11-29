use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_common_assets::json::JsonAssetPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            JsonAssetPlugin::<Level>::new(&["level.json"]),
        ))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_level.run_if(in_state(AppState::Loading)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level = LevelHandle(asset_server.load("trees.level.json"));
    commands.insert_resource(level);
    let tree = ImageHandle(asset_server.load("tree.png"));
    commands.insert_resource(tree);

    commands.spawn((Camera2d, Msaa::Off));
}

fn spawn_level(
    mut commands: Commands,
    level: Res<LevelHandle>,
    tree: Res<ImageHandle>,
    mut levels: ResMut<Assets<Level>>,
    mut state: ResMut<NextState<AppState>>,
) {
    if let Some(level) = levels.remove(level.0.id()) {
        for position in level.positions {
            commands.spawn((
                Sprite::from_image(tree.0.clone()),
                Transform::from_translation(position.into()),
            ));
        }

        state.set(AppState::Level);
    }
}

#[derive(serde::Deserialize, Asset, TypePath)]
struct Level {
    positions: Vec<[f32; 3]>,
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
struct LevelHandle(Handle<Level>);
