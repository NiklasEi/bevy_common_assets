use bevy::asset::processor::LoadTransformAndSave;
use bevy::asset::transformer::IdentityAssetTransformer;
use bevy::prelude::*;
use bevy_common_assets::json::{JsonAssetLoader, JsonAssetPlugin};
use bevy_common_assets::postcard::{PostcardAssetPlugin, PostcardAssetSaver};
use serde::{Deserialize, Serialize};

/// This example processes a json level asset into a postcard asset (binary format)
/// which is then loaded and rendered. If you run the example, the directory
/// `examples/asset_savers/imported_assets` is created and populated.
///
/// Take a look at `examples/asset_savers/assets/trees.level.meta` to see the configuration
/// that tells Bevy which processor to use to convert the json asset into the postcard format.

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                mode: AssetMode::Processed,
                file_path: "examples/asset_savers/assets".to_string(),
                processed_file_path: "examples/asset_savers/imported_assets/Default".to_string(),
                ..default()
            }),
            PostcardAssetPlugin::<Level>::new(&["level"]),
            JsonAssetPlugin::<Level>::new(&[]),
        ))
        .register_asset_processor::<LoadTransformAndSave<
            JsonAssetLoader<Level>,
            IdentityAssetTransformer<Level>,
            PostcardAssetSaver<Level>,
        >>(LoadTransformAndSave::new(
            IdentityAssetTransformer::<Level>::default(),
            PostcardAssetSaver::<Level>::default(),
        ))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_level.run_if(in_state(AppState::Loading)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level = LevelHandle(asset_server.load("trees.level"));
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

#[derive(Deserialize, Serialize, Asset, TypePath)]
struct Level {
    positions: Vec<[f32; 3]>,
}
