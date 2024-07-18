use bevy::app::{App, Plugin};
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetApp, AssetLoader, AsyncReadExt, LoadContext};
use serde_json::from_str;
use std::io::BufRead;
use std::marker::PhantomData;
use thiserror::Error;

/// Plugin to load your asset type `A` via a list of ` from jsonl files.
pub struct JsonLinesAssetPlugin<A, D> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<(A, D)>,
}

impl<A, D> Plugin for JsonLinesAssetPlugin<A, D>
where
    for<'de> D: serde::Deserialize<'de> + Sync + Send + 'de,
    for<'de> A: FromIterator<D> + Asset + Sync + Send + 'de,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<A>()
            .register_asset_loader(JsonLinesAssetLoader::<A, D> {
                extensions: self.extensions.clone(),
                _marker: PhantomData,
            });
    }
}

impl<A, D> JsonLinesAssetPlugin<A, D>
where
    for<'de> D: serde::Deserialize<'de> + Sync + Send + 'de,
    for<'de> A: FromIterator<D> + Asset + Sync + Send + 'de,
{
    /// Create a new plugin that will load assets from files with the given extensions.
    pub fn new(extensions: &[&'static str]) -> Self {
        Self {
            extensions: extensions.to_owned(),
            _marker: PhantomData,
        }
    }
}

/// Loads your asset type `A` from jsonl files
pub struct JsonLinesAssetLoader<A, D> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<(A, D)>,
}

/// Possible errors that can be produced by [`JsonLinesAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum JsonLinesLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [JSON Error](serde_json::error::Error)
    #[error("Could not parse the JSON: {0}")]
    JsonError(#[from] serde_json::error::Error),
}

impl<A, D> AssetLoader for JsonLinesAssetLoader<A, D>
where
    for<'de> D: serde::Deserialize<'de> + Sync + Send + 'de,
    for<'de> A: FromIterator<D> + Asset + Sync + Send + 'de,
{
    type Asset = A;
    type Settings = ();
    type Error = JsonLinesLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        if let Some(b'\n') = bytes.last() {
            // Json Lines may optionally end with a line break.
            let _ = bytes.pop();
        }

        bytes
            .lines()
            .map(|line| Ok::<D, JsonLinesLoaderError>(from_str(line?.as_str())?))
            .collect()
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}
