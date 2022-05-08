//! Bevy plugin offering generic asset loaders for common file formats
//!
//! This library includes a collection of thin wrapper plugins around serde implementations for the
//! common file formats json, ron, toml, yaml, and MessagePack. Each plugin adds an asset loader
//! for a user type. Assets of that type will then be loaded from all files with configurable
//! extensions.
//!
//! The following example requires the `json` feature and loads a custom asset from a json file.
//! ```
//! use bevy::prelude::*;
//! # /*
//! use bevy_common_assets::json::JsonAssetPlugin;
//! # */
//! # use bevy::app::AppExit;
//!
//! fn main() {
//!     App::new()
//! # /*
//!         .add_plugins(DefaultPlugins)
//! # */
//! #       .add_plugins(MinimalPlugins)
//! #       .add_plugin(bevy::asset::AssetPlugin::default())
//! # /*
//!         .add_plugin(JsonAssetPlugin::<Level>::new(&["level"]))
//! # */
//!         .add_startup_system(load_level)
//! #       .add_system(stop)
//!         .run()
//! }
//!
//! fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
//!     let handle: Handle<Level> = asset_server.load("trees.level");
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
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Module containing a Bevy plugin to load assets from json files with custom file extensions.
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
#[cfg(feature = "json")]
pub mod json;
/// Module containing a Bevy plugin to load assets from MassagePack files with custom file extensions.
#[cfg_attr(docsrs, doc(cfg(feature = "msgpack")))]
#[cfg(feature = "msgpack")]
pub mod msgpack;
/// Module containing a Bevy plugin to load assets from ron files with custom file extensions.
#[cfg_attr(docsrs, doc(cfg(feature = "ron")))]
#[cfg(feature = "ron")]
pub mod ron;
/// Module containing a Bevy plugin to load assets from toml files with custom file extensions.
#[cfg_attr(docsrs, doc(cfg(feature = "toml")))]
#[cfg(feature = "toml")]
pub mod toml;
/// Module containing a Bevy plugin to load assets from yaml files with custom file extensions.
#[cfg_attr(docsrs, doc(cfg(feature = "yaml")))]
#[cfg(feature = "yaml")]
pub mod yaml;

#[cfg(all(
    feature = "json",
    feature = "msgpack",
    feature = "ron",
    feature = "toml",
    feature = "yaml"
))]
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
