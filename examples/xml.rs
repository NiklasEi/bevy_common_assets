use bevy::math::f32::Vec3;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_common_assets::xml::XmlAssetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(XmlAssetPlugin::<Level>::new(&["level.xml"]))
        .insert_resource(Msaa::Off)
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_level.run_if(in_state(AppState::Loading)))
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level = LevelHandle(asset_server.load("trees.level.xml"));
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

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
struct Level {
    #[serde(rename = "Position")]
    positions: Vec<Position>,
}

#[derive(serde::Deserialize)]
struct Position {
    #[serde(rename = "@x")]
    x: f32,
    #[serde(rename = "@y")]
    y: f32,
    #[serde(rename = "@z")]
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
struct LevelHandle(Handle<Level>);
