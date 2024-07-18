use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_common_assets::jsonl::JsonLinesAssetPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            JsonLinesAssetPlugin::<Level, TreePosition>::new(&["level.jsonl"]),
        ))
        .insert_resource(Msaa::Off)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_level.run_if(in_state(AppState::Loading)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level = LevelHandle(asset_server.load("trees.level.jsonl"));
    commands.insert_resource(level);
    let tree = ImageHandle(asset_server.load("tree.png"));
    commands.insert_resource(tree);

    commands.spawn(Camera2dBundle::default());
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
            commands.spawn(SpriteBundle {
                transform: Transform::from_translation(position.0.into()),
                texture: tree.0.clone(),
                ..default()
            });
        }

        state.set(AppState::Level);
    }
}

#[derive(serde::Deserialize)]
#[serde(transparent)]
struct TreePosition([f32; 3]);

#[derive(serde::Deserialize, Asset, TypePath)]
struct Level {
    positions: Vec<TreePosition>,
}

impl FromIterator<TreePosition> for Level {
    fn from_iter<T: IntoIterator<Item = TreePosition>>(iter: T) -> Self {
        let positions = Vec::from_iter(iter);
        Self { positions }
    }
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
