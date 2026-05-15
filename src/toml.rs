use bevy_app::{App, Plugin};
use bevy_asset::io::Reader;
use bevy_asset::{Asset, AssetApp, AssetLoader, AsyncWriteExt, LoadContext, saver::AssetSaver};
use bevy_reflect::TypePath;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::str::from_utf8;
use thiserror::Error;

/// Plugin to load your asset type `A` from toml files.
pub struct TomlAssetPlugin<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

impl<A> Plugin for TomlAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<A>()
            .register_asset_loader(TomlAssetLoader::<A> {
                extensions: self.extensions.clone(),
                _marker: PhantomData,
            });
    }
}

impl<A> TomlAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    /// Create a new plugin that will load assets from files with the given extensions.
    pub fn new(extensions: &[&'static str]) -> Self {
        Self {
            extensions: extensions.to_owned(),
            _marker: PhantomData,
        }
    }
}

/// Loads your asset type `A` from toml files
#[derive(TypePath)]
pub struct TomlAssetLoader<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

/// Possible errors that can be produced by [`TomlAssetLoader`] or [`TomlAssetSaver`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum TomlAssetError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [conversion Error](std::str::Utf8Error)
    #[error("Could not interpret as UTF-8: {0}")]
    FormatError(#[from] std::str::Utf8Error),
    /// A [TOML deserialization Error](serde_toml::de::Error)
    #[error("Could not parse TOML: {0}")]
    TomlDeError(#[from] serde_toml::de::Error),
    /// A [TOML serialization Error](serde_toml::ser::Error)
    #[error("Could not serialize TOML: {0}")]
    TomlSerError(#[from] serde_toml::ser::Error),
}

/// Deprecated alias for [`TomlAssetError`]
#[deprecated(since = "0.15.0", note = "Use TomlAssetError instead")]
pub type TomlLoaderError = TomlAssetError;

impl<A> AssetLoader for TomlAssetLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    type Asset = A;
    type Settings = ();
    type Error = TomlAssetError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = serde_toml::from_str::<A>(from_utf8(&bytes)?)?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}

/// Saves your asset type `A` to TOML files
#[derive(TypePath)]
pub struct TomlAssetSaver<A> {
    _marker: PhantomData<A>,
}

impl<A> Default for TomlAssetSaver<A> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<A: Asset + for<'de> Deserialize<'de> + Serialize> AssetSaver for TomlAssetSaver<A> {
    type Asset = A;
    type Settings = ();
    type OutputLoader = TomlAssetLoader<A>;
    type Error = TomlAssetError;

    async fn save(
        &self,
        writer: &mut bevy_asset::io::Writer,
        asset: bevy_asset::saver::SavedAsset<'_,'_, Self::Asset>,
        _settings: &Self::Settings,
        _asset_path: bevy_asset::AssetPath<'_>,
    ) -> Result<<Self::OutputLoader as AssetLoader>::Settings, Self::Error> {
        let toml = serde_toml::to_string(asset.get())?;
        writer.write_all(toml.as_bytes()).await?;
        Ok(())
    }
}
