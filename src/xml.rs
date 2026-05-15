use bevy_app::{App, Plugin};
use bevy_asset::io::Reader;
use bevy_asset::{Asset, AssetApp, AssetLoader, AsyncWriteExt, LoadContext, saver::AssetSaver};
use bevy_reflect::TypePath;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::str::from_utf8;
use thiserror::Error;

/// Plugin to load your asset type `A` from xml files.
/// Read the [`quick_xml` docs](https://docs.rs/quick-xml/latest/quick_xml/de/) for tips on deserialization.
pub struct XmlAssetPlugin<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

impl<A> Plugin for XmlAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<A>()
            .register_asset_loader(XmlAssetLoader::<A> {
                extensions: self.extensions.clone(),
                _marker: PhantomData,
            });
    }
}

impl<A> XmlAssetPlugin<A>
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

/// Loads your asset type `A` from xml files
#[derive(TypePath)]
pub struct XmlAssetLoader<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

/// Possible errors that can be produced by [`XmlAssetLoader`] or [`XmlAssetSaver`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum XmlAssetError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [conversion Error](std::str::Utf8Error)
    #[error("Could not interpret as UTF-8: {0}")]
    FormatError(#[from] std::str::Utf8Error),
    /// A [XML deserialization Error](quick_xml::DeError)
    #[error("Could not parse XML: {0}")]
    XmlDeError(#[from] quick_xml::DeError),
    /// A XML serialization error
    #[error("Could not serialize XML: {0}")]
    XmlSerError(String),
}

/// Deprecated alias for [`XmlAssetError`]
#[deprecated(since = "0.15.0", note = "Use XmlAssetError instead")]
pub type XmlLoaderError = XmlAssetError;

impl<A> AssetLoader for XmlAssetLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    type Asset = A;
    type Settings = ();
    type Error = XmlAssetError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = from_str::<A>(from_utf8(&bytes)?)?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}

/// Saves your asset type `A` to XML files
#[derive(TypePath)]
pub struct XmlAssetSaver<A> {
    _marker: PhantomData<A>,
}

impl<A> Default for XmlAssetSaver<A> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<A: Asset + for<'de> Deserialize<'de> + Serialize> AssetSaver for XmlAssetSaver<A> {
    type Asset = A;
    type Settings = ();
    type OutputLoader = XmlAssetLoader<A>;
    type Error = XmlAssetError;

    async fn save(
        &self,
        writer: &mut bevy_asset::io::Writer,
        asset: bevy_asset::saver::SavedAsset<'_, '_, Self::Asset>,
        _settings: &Self::Settings,
        _asset_path: bevy_asset::AssetPath<'_>,
    ) -> Result<<Self::OutputLoader as AssetLoader>::Settings, Self::Error> {
        let xml = quick_xml::se::to_string(asset.get())
            .map_err(|e| XmlAssetError::XmlSerError(e.to_string()))?;
        writer.write_all(xml.as_bytes()).await?;
        Ok(())
    }
}
