use bevy_app::{App, Plugin};
use bevy_asset::io::Reader;
use bevy_asset::{Asset, AssetApp, AssetLoader, AsyncWriteExt, LoadContext, saver::AssetSaver};
use bevy_reflect::TypePath;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use std::marker::PhantomData;
use thiserror::Error;

/// Plugin to load your asset type `A` from json files.
pub struct JsonAssetPlugin<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

impl<A> Plugin for JsonAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<A>()
            .register_asset_loader(JsonAssetLoader::<A> {
                extensions: self.extensions.clone(),
                _marker: PhantomData,
            });
    }
}

impl<A> JsonAssetPlugin<A>
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

/// Loads your asset type `A` from json files
#[derive(TypePath)]
pub struct JsonAssetLoader<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

/// Possible errors that can be produced by [`JsonAssetLoader`] or [`JsonAssetSaver`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum JsonAssetError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [JSON Error](serde_json::error::Error)
    #[error("Could not parse/serialize JSON: {0}")]
    JsonError(#[from] serde_json::error::Error),
}

/// Deprecated alias for [`JsonAssetError`]
#[deprecated(since = "0.15.0", note = "Use JsonAssetError instead")]
pub type JsonLoaderError = JsonAssetError;

impl<A> AssetLoader for JsonAssetLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    type Asset = A;
    type Settings = ();
    type Error = JsonAssetError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = from_slice::<A>(&bytes)?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}

/// Saves your asset type `A` to JSON files
#[derive(TypePath)]
pub struct JsonAssetSaver<A> {
    _marker: PhantomData<A>,
}

impl<A> Default for JsonAssetSaver<A> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<A: Asset + for<'de> Deserialize<'de> + Serialize> AssetSaver for JsonAssetSaver<A> {
    type Asset = A;
    type Settings = ();
    type OutputLoader = JsonAssetLoader<A>;
    type Error = JsonAssetError;

    async fn save(
        &self,
        writer: &mut bevy_asset::io::Writer,
        asset: bevy_asset::saver::SavedAsset<'_, '_, Self::Asset>,
        _settings: &Self::Settings,
        _asset_path: bevy_asset::AssetPath<'_>,
    ) -> Result<<Self::OutputLoader as AssetLoader>::Settings, Self::Error> {
        let bytes = serde_json::to_vec(asset.get())?;
        writer.write_all(&bytes).await?;
        Ok(())
    }
}
