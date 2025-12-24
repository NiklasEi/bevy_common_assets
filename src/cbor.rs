use bevy_app::{App, Plugin};
use bevy_asset::{
    Asset, AssetApp, AssetLoader, AsyncWriteExt, LoadContext, io::Reader, saver::AssetSaver,
};
use ciborium::from_reader;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use thiserror::Error;

/// Plugin to load your asset type `A` from "Concise Binary Object Representation" (CBOR) files.
pub struct CborAssetPlugin<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

impl<A> Plugin for CborAssetPlugin<A>
where
    for<'de> A: Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<A>()
            .register_asset_loader(CborAssetLoader::<A> {
                extensions: self.extensions.clone(),
                _marker: PhantomData,
            });
    }
}

impl<A> CborAssetPlugin<A>
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

/// Loads your asset type `A` from CBOR files
pub struct CborAssetLoader<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

/// Possible errors that can be produced by [`CborAssetLoader`] or [`CborAssetSaver`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CborAssetError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),

    /// A [ciborium serializing Error](ciborium::ser::Error)
    #[error("Could not serialize into CBOR: {0}")]
    CborSerError(#[from] ciborium::ser::Error<std::io::Error>),

    /// A [ciborium deserializing Error](ciborium::de::Error)
    #[error("Could not parse CBOR: {0}")]
    CborDeError(#[from] ciborium::de::Error<std::io::Error>),
}

impl<A> AssetLoader for CborAssetLoader<A>
where
    for<'de> A: Deserialize<'de> + Asset,
{
    type Asset = A;
    type Settings = ();
    type Error = CborAssetError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset: A = from_reader(&bytes[..])?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}

/// Saves your asset type `A` to `Cbor` files
pub struct CborAssetSaver<A> {
    _marker: PhantomData<A>,
}

impl<A> Default for CborAssetSaver<A> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<A: Asset + for<'de> Deserialize<'de> + Serialize> AssetSaver for CborAssetSaver<A> {
    type Asset = A;
    type Settings = ();
    type OutputLoader = CborAssetLoader<A>;
    type Error = CborAssetError;

    async fn save(
        &self,
        writer: &mut bevy_asset::io::Writer,
        asset: bevy_asset::saver::SavedAsset<'_, Self::Asset>,
        _settings: &Self::Settings,
    ) -> Result<<Self::OutputLoader as AssetLoader>::Settings, Self::Error> {
        let mut bytes = Vec::new();
        ciborium::into_writer(&asset.get(), &mut bytes)?;
        writer.write_all(&bytes).await?;
        Ok(())
    }
}
