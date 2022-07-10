use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_common_assets::yaml::YamlAssetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(YamlAssetPlugin::<Level>::new(&["yaml.level"]))
        .insert_resource(Msaa { samples: 1 })
        .add_state(AppState::Loading)
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(spawn_level))
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<Level> = asset_server.load("trees.yaml.level");
    commands.insert_resource(handle);
    let tree: Handle<Image> = asset_server.load("tree.png");
    commands.insert_resource(tree);

    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_level(
    mut commands: Commands,
    handle: Res<Handle<Level>>,
    mut levels: ResMut<Assets<Level>>,
    mut state: ResMut<State<AppState>>,
    tree: Res<Handle<Image>>,
) {
    if let Some(level) = levels.remove(handle.id) {
        for position in level.positions {
            commands.spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(position.into()),
                texture: tree.clone(),
                ..default()
            });
        }

        state.set(AppState::Level).unwrap();
    }
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
struct Level {
    positions: Vec<[f32; 3]>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    Loading,
    Level,
}
