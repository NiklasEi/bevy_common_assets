use bevy::{
    app::{App, Plugin},
    asset::{
        io::Reader, saver::AssetSaver, Asset, AssetApp, AssetLoader, AsyncWriteExt, LoadContext,
    },
    prelude::*,
};
use postcard::{from_bytes, to_stdvec};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use thiserror::Error;

/// Plugin to load your asset type `A` from `Postcard` files.
pub struct PostcardAssetPlugin<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

impl<A> Plugin for PostcardAssetPlugin<A>
where
    for<'de> A: Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<A>()
            .register_asset_loader(PostcardAssetLoader::<A> {
                extensions: self.extensions.clone(),
                _marker: PhantomData,
            });
    }
}

impl<A> PostcardAssetPlugin<A>
where
    for<'de> A: Deserialize<'de> + Asset,
{
    /// Create a new plugin that will load assets from files with the given extensions.
    pub fn new(extensions: &[&'static str]) -> Self {
        Self {
            extensions: extensions.to_owned(),
            _marker: PhantomData,
        }
    }
}

/// Loads your asset type `A` from `Postcard` files
pub struct PostcardAssetLoader<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

/// Possible errors that can be produced by [`PostcardAssetLoader`] or [`PostcardAssetSaver`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PostcardAssetError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [Postcard Error](postcard::Error)
    #[error("Could not parse Postcard: {0}")]
    PostcardError(#[from] postcard::Error),
}

impl<A> AssetLoader for PostcardAssetLoader<A>
where
    for<'de> A: Deserialize<'de> + Asset,
{
    type Asset = A;
    type Settings = ();
    type Error = PostcardAssetError;

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

/// Saves your asset type `A` to `Postcard` files
pub struct PostcardAssetSaver<A> {
    _marker: PhantomData<A>,
}

impl<A> Default for PostcardAssetSaver<A> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<A: Asset + for<'de> Deserialize<'de> + Serialize> AssetSaver for PostcardAssetSaver<A> {
    type Asset = A;
    type Settings = ();
    type OutputLoader = PostcardAssetLoader<A>;
    type Error = PostcardAssetError;

    async fn save(
        &self,
        writer: &mut bevy::asset::io::Writer,
        asset: bevy::asset::saver::SavedAsset<'_, Self::Asset>,
        _settings: &Self::Settings,
    ) -> Result<<Self::OutputLoader as AssetLoader>::Settings, Self::Error> {
        let bytes = to_stdvec(&asset.get())?;
        writer.write_all(&bytes).await?;
        Ok(())
    }
}
