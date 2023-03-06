use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_common_assets::json::JsonAssetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(JsonAssetPlugin::<Level>::new(&["level.json"]))
        .insert_resource(Msaa::Off)
        .add_state::<AppState>()
        .add_system(setup.on_startup())
        .add_system(spawn_level.run_if(in_state(AppState::Loading)))
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level = LevelHandle(asset_server.load("trees.level.json"));
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
                transform: Transform::from_translation(position.into()),
                texture: tree.0.clone(),
                ..default()
            });
        }

        state.set(AppState::Level);
    }
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
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
