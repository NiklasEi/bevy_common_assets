//! Bevy plugin offering generic asset loaders for common file formats
//!
//! This library includes a collection of thin wrapper plugins around serde implementations for
//! common file formats like json, ron, toml, and yaml. Each plugin adds an asset loader for a user
//! type. Assets of that type will then be loaded from all files with configurable extensions.
//!
//! ```
//! use bevy::prelude::*;
//! use bevy_common_assets::json::JsonAssetPlugin;
//! # use bevy::app::AppExit;
//!
//! fn main() {
//!     App::new()
//! # /*
//!         .add_plugins(DefaultPlugins)
//! # */
//! #       .add_plugins(MinimalPlugins)
//! #       .add_plugin(bevy::asset::AssetPlugin::default())
//!         .add_plugin(JsonAssetPlugin::<Level>::new(&["level"]))
//!         .add_startup_system(load_level)
//! #       .add_system(stop)
//!         .run()
//! }
//!
//! fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
//!     let handle: Handle<Level> = asset_server.load("trees.json.level");
//!     commands.insert_resource(handle);
//! }
//!
//! #[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
//! #[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
//! struct Level {
//!     positions: Vec<[f32; 3]>,
//! }
//!
//! # fn stop(mut events: EventWriter<AppExit>) {
//! #     events.send(AppExit)
//! # }
//! ```

#![forbid(unsafe_code)]
#![warn(unused_imports, missing_docs)]

/// Module containing a Bevy plugin to load assets from json files with custom file extensions.
#[cfg(feature = "json")]
pub mod json;
/// Module containing a Bevy plugin to load assets from ron files with custom file extensions.
#[cfg(feature = "ron")]
pub mod ron;
/// Module containing a Bevy plugin to load assets from toml files with custom file extensions.
#[cfg(feature = "toml")]
pub mod toml;
/// Module containing a Bevy plugin to load assets from yaml files with custom file extensions.
#[cfg(feature = "yaml")]
pub mod yaml;
