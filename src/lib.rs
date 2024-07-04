//! Bevy plugin offering generic asset loaders for common file formats
//!
//! This library includes a collection of thin wrapper plugins around serde implementations for the
//! common file formats `json`, `ron`, `toml`, `yaml`, `MessagePack` and `xml`. Each plugin adds
//! an asset loader for a user type. Assets of that type will then be loaded from all files with
//! configurable extensions.
//!
//! The following example requires the `json` feature and loads a custom asset from a json file.
//! ```
//! use bevy::prelude::*;
//! use bevy::reflect::TypePath;
//! # /*
//! use bevy_common_assets::json::JsonAssetPlugin;
//! # */
//! # use bevy::app::AppExit;
//!
//! fn main() {
//!     App::new()
//! # /*
//!         .add_plugins((DefaultPlugins, JsonAssetPlugin::<Level>::new(&["level.json"])))
//! # */
//! #       .add_plugins((MinimalPlugins, AssetPlugin::default()))
//! #       .init_asset::<Level>()
//!         .add_systems(Startup, load_level)
//! #       .add_systems(Update, stop)
//!         .run();
//! }
//!
//! fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
//!     let handle = LevelAsset(asset_server.load("trees.level.json"));
//!     commands.insert_resource(handle);
//! }
//!
//! #[derive(serde::Deserialize, Asset, TypePath)]
//! struct Level {
//!     positions: Vec<[f32; 3]>,
//! }
//!
//! #[derive(Resource)]
//! struct LevelAsset(Handle<Level>);
//!
//! # fn stop(mut events: EventWriter<AppExit>) {
//! #     events.send(AppExit::Success);
//! # }
//! ```

#![forbid(unsafe_code)]
#![warn(unused_imports, missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Module containing a Bevy plugin to load assets from `csv` files with custom file extensions.
#[cfg_attr(docsrs, doc(cfg(feature = "csv")))]
#[cfg(feature = "csv")]
pub mod csv;
/// Module containing a Bevy plugin to load assets from `json` files with custom file extensions.
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
#[cfg(feature = "json")]
pub mod json;
/// Module containing a Bevy plugin to load assets from `MessagePack` files with custom file extensions.
#[cfg_attr(docsrs, doc(cfg(feature = "msgpack")))]
#[cfg(feature = "msgpack")]
pub mod msgpack;
/// Module containing a Bevy plugin to load assets from `postcard` files with custom file extensions.
#[cfg_attr(docsrs, doc(cfg(feature = "postcard")))]
#[cfg(feature = "postcard")]
pub mod postcard;
/// Module containing a Bevy plugin to load assets from `ron` files with custom file extensions.
#[cfg_attr(docsrs, doc(cfg(feature = "ron")))]
#[cfg(feature = "ron")]
pub mod ron;
/// Module containing a Bevy plugin to load assets from `toml` files with custom file extensions.
#[cfg_attr(docsrs, doc(cfg(feature = "toml")))]
#[cfg(feature = "toml")]
pub mod toml;
/// Module containing a Bevy plugin to load assets from `xml` files with custom file extensions.
#[cfg_attr(docsrs, doc(cfg(feature = "xml")))]
#[cfg(feature = "xml")]
pub mod xml;
/// Module containing a Bevy plugin to load assets from `yaml` files with custom file extensions.
#[cfg_attr(docsrs, doc(cfg(feature = "yaml")))]
#[cfg(feature = "yaml")]
pub mod yaml;

#[cfg(all(
    feature = "json",
    feature = "msgpack",
    feature = "ron",
    feature = "toml",
    feature = "xml",
    feature = "yaml",
    feature = "csv",
    feature = "postcard",
))]
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
