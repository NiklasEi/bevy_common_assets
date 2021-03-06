use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_common_assets::ron::RonAssetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // You can add loaders for different asset types, but also multiple loaders for the same asset type
        // The important thing is: they all need distinct extensions!
        .add_plugin(JsonAssetPlugin::<Level>::new(&["json.level"]))
        .add_plugin(RonAssetPlugin::<Level>::new(&["ron.level"]))
        .insert_resource(Msaa { samples: 1 })
        .add_state(AppState::Loading)
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_loading))
        .add_system_set(SystemSet::on_enter(AppState::Level).with_system(spawn_level))
        .run();
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
struct Level {
    positions: Vec<[f32; 3]>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let json_trees: Handle<Level> = asset_server.load("trees.json.level");
    let ron_trees: Handle<Level> = asset_server.load("trees.ron.level");
    commands.insert_resource(Handles(vec![json_trees, ron_trees]));
    let tree: Handle<Image> = asset_server.load("tree.png");
    commands.insert_resource(tree);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_level(
    mut commands: Commands,
    handles: Res<Handles>,
    mut levels: ResMut<Assets<Level>>,
    tree: Res<Handle<Image>>,
) {
    for handle in handles.0.iter() {
        let level = levels.remove(handle).unwrap();
        for position in level.positions {
            commands.spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(position.into()),
                texture: tree.clone(),
                ..default()
            });
        }
    }
}

struct Handles(Vec<Handle<Level>>);

fn check_loading(
    asset_server: Res<AssetServer>,
    handles: Res<Handles>,
    mut state: ResMut<State<AppState>>,
) {
    if asset_server.get_group_load_state(handles.0.iter().map(|handle| handle.id))
        == LoadState::Loaded
    {
        state.set(AppState::Level).unwrap();
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    Loading,
    Level,
}
