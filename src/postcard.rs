use bevy::{
    app::{App, Plugin},
    asset::{
        io::Reader, saver::AssetSaver, Asset, AssetApp, AssetLoader, AsyncReadExt, AsyncWriteExt,
        LoadContext,
    },
    prelude::*,
    utils::{thiserror, BoxedFuture},
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

struct PostcardAssetLoader<A> {
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

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let asset = from_bytes::<A>(&bytes)?;
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}

struct PostcardAssetSaver<A> {
    _marker: PhantomData<A>,
}

impl<A: Asset + Serialize> AssetSaver for PostcardAssetSaver<A> {
    type Asset = A;
    type Settings = ();
    type OutputLoader = ();
    type Error = PostcardAssetError;

    fn save<'a>(
        &'a self,
        writer: &'a mut bevy::asset::io::Writer,
        asset: bevy::asset::saver::SavedAsset<'a, Self::Asset>,
        _settings: &'a Self::Settings,
    ) -> BoxedFuture<'a, Result<<Self::OutputLoader as AssetLoader>::Settings, Self::Error>> {
        Box::pin(async move {
            let bytes = to_stdvec(&asset.get())?;
            writer.write_all(&bytes).await?;
            Ok(())
        })
    }
}
