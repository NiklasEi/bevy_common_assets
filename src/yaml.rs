use bevy_app::{App, Plugin};
use bevy_asset::io::Reader;
use bevy_asset::{Asset, AssetApp, AssetLoader, LoadContext};
use bevy_reflect::TypePath;
use serde_yaml::from_slice;
use std::marker::PhantomData;
use thiserror::Error;

/// Plugin to load your asset type `A` from yaml files.
pub struct YamlAssetPlugin<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

impl<A> Plugin for YamlAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<A>()
            .register_asset_loader(YamlAssetLoader::<A> {
                extensions: self.extensions.clone(),
                _marker: PhantomData,
            });
    }
}

impl<A> YamlAssetPlugin<A>
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

/// Loads your asset type `A` from yaml files
#[derive(TypePath)]
pub struct YamlAssetLoader<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

/// Possible errors that can be produced by [`YamlAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum YamlLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [YAML Error](serde_yaml::Error)
    #[error("Could not parse YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

impl<A> AssetLoader for YamlAssetLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    type Asset = A;
    type Settings = ();
    type Error = YamlLoaderError;

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
