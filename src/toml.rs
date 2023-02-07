use bevy::app::{App, Plugin};
use bevy::asset::{AddAsset, Asset, AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use std::marker::PhantomData;
use std::str::from_utf8;

/// Plugin to load your asset type `A` from toml files.
pub struct TomlAssetPlugin<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

impl<A> Plugin for TomlAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.add_asset::<A>().add_asset_loader(TomlAssetLoader::<A> {
            extensions: self.extensions.clone(),
            _marker: PhantomData,
        });
    }
}

impl<A> TomlAssetPlugin<A>
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

struct TomlAssetLoader<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

impl<A> AssetLoader for TomlAssetLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let asset = serde_toml::from_str::<A>(from_utf8(bytes)?)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}
