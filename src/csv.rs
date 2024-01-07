use bevy::app::{App, Plugin};
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetApp, AssetLoader, AsyncReadExt, BoxedFuture, Handle, LoadContext};
use bevy::prelude::TypePath;
use std::marker::PhantomData;
use thiserror::Error;

/// Plugin to load your asset type `A` from csv files.
pub struct CsvAssetPlugin<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

impl<A> Plugin for CsvAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<A>()
            .init_asset::<LoadedCsv<A>>()
            .register_asset_loader(CsvAssetLoader::<A> {
                extensions: self.extensions.clone(),
                _marker: PhantomData,
            });
    }
}

impl<A> CsvAssetPlugin<A>
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

struct CsvAssetLoader<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

/// Possible errors that can be produced by [`CsvAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CsvLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [CSV Error](serde_csv::Error)
    #[error("Could not parse CSV: {0}")]
    CsvError(#[from] csv::Error),
}

/// Asset representing a loaded CSV file with rows deserialized to Assets of type `A`
#[derive(TypePath, Asset)]
pub struct LoadedCsv<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    /// Handles to the Assets the were loaded from the rows of this CSV file
    pub rows: Vec<Handle<A>>,
}

impl<A> AssetLoader for CsvAssetLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    type Asset = LoadedCsv<A>;
    type Settings = ();
    type Error = CsvLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let mut reader = csv::Reader::from_reader(bytes.as_slice());
            let mut handles = vec![];
            for (index, result) in reader.deserialize().enumerate() {
                let asset: A = result?;
                handles
                    .push(load_context.add_loaded_labeled_asset(index.to_string(), asset.into()));
            }
            Ok(LoadedCsv { rows: handles })
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}
