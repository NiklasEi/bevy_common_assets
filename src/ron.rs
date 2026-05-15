use bevy_app::{App, Plugin};
use bevy_asset::io::Reader;
use bevy_asset::{Asset, AssetApp, AssetLoader, AsyncWriteExt, LoadContext, saver::AssetSaver};
use bevy_reflect::TypePath;
use serde::{Deserialize, Serialize};
use serde_ron::de::from_bytes;
use std::marker::PhantomData;
use thiserror::Error;

/// Plugin to load your asset type `A` from ron files.
pub struct RonAssetPlugin<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

impl<A> Plugin for RonAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<A>()
            .register_asset_loader(RonAssetLoader::<A> {
                extensions: self.extensions.clone(),
                _marker: PhantomData,
            });
    }
}

impl<A> RonAssetPlugin<A>
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

/// Loads your asset type `A` from ron files
#[derive(TypePath)]
pub struct RonAssetLoader<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

/// Possible errors that can be produced by [`RonAssetLoader`] or [`RonAssetSaver`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum RonAssetError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON deserialization Error](serde_ron::error::SpannedError)
    #[error("Could not parse RON: {0}")]
    RonDeError(#[from] serde_ron::error::SpannedError),
    /// A [RON serialization Error](serde_ron::Error)
    #[error("Could not serialize RON: {0}")]
    RonSerError(#[from] serde_ron::Error),
}

/// Deprecated alias for [`RonAssetError`]
#[deprecated(since = "0.15.0", note = "Use RonAssetError instead")]
pub type RonLoaderError = RonAssetError;

impl<A> AssetLoader for RonAssetLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    type Asset = A;
    type Settings = ();
    type Error = RonAssetError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = from_bytes::<A>(&bytes)?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}

/// Saves your asset type `A` to RON files
#[derive(TypePath)]
pub struct RonAssetSaver<A> {
    _marker: PhantomData<A>,
}

impl<A> Default for RonAssetSaver<A> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<A: Asset + for<'de> Deserialize<'de> + Serialize> AssetSaver for RonAssetSaver<A> {
    type Asset = A;
    type Settings = ();
    type OutputLoader = RonAssetLoader<A>;
    type Error = RonAssetError;

    async fn save(
        &self,
        writer: &mut bevy_asset::io::Writer,
        asset: bevy_asset::saver::SavedAsset<'_, '_, Self::Asset>,
        _settings: &Self::Settings,
        _asset_path: bevy_asset::AssetPath<'_>,
    ) -> Result<<Self::OutputLoader as AssetLoader>::Settings, Self::Error> {
        let ron = serde_ron::ser::to_string(asset.get())?;
        writer.write_all(ron.as_bytes()).await?;
        Ok(())
    }
}
