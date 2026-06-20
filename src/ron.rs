use bevy_app::{App, Plugin};
use bevy_asset::io::Reader;
use bevy_asset::{Asset, AssetApp, AssetLoader, AsyncWriteExt, LoadContext, saver::AssetSaver};
use bevy_reflect::TypePath;
use serde::{Deserialize, Serialize};
use serde_ron::{Options, extensions::Extensions};
use std::marker::PhantomData;
use thiserror::Error;

/// Plugin to load your asset type `A` from ron files.
pub struct RonAssetPlugin<A> {
    extensions: Vec<&'static str>,
    options: Options,
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
                options: self.options.clone(),
                _marker: PhantomData,
            });
    }
}

impl<A> RonAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    /// Create a new plugin that will load assets from files with the given extensions.
    ///
    /// Enables [`Extensions::IMPLICIT_SOME`] by default.
    pub fn new(extensions: &[&'static str]) -> Self {
        Self {
            extensions: extensions.to_owned(),
            options: Options::default().with_default_extension(Extensions::IMPLICIT_SOME),
            _marker: PhantomData,
        }
    }

    /// Customize RON deserialization options.
    ///
    /// ```no_run
    /// # use bevy::prelude::*;
    /// # use bevy_common_assets::ron::RonAssetPlugin;
    /// use serde_ron::{extensions::Extensions, Options};
    ///
    /// App::new()
    ///     .add_plugins(RonAssetPlugin::<Level>::new(&["level.ron"])
    ///         .with_options(Options::default().with_default_extension(
    ///             Extensions::UNWRAP_NEWTYPES | Extensions::IMPLICIT_SOME,
    ///         )));
    /// # #[derive(serde::Deserialize, Asset, TypePath)]
    /// # struct Level {
    /// #     value: Option<f32>,
    /// # }
    /// ```
    pub fn with_options(mut self, options: Options) -> Self {
        self.options = options;
        self
    }
}

/// Loads your asset type `A` from ron files
#[derive(TypePath)]
pub struct RonAssetLoader<A> {
    extensions: Vec<&'static str>,
    options: Options,
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
        let asset = self.options.from_bytes::<A>(&bytes)?;
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
