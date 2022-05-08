use bevy::app::{App, Plugin};
use bevy::asset::{AddAsset, Asset, AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use rmp_serde::from_slice;
use std::marker::PhantomData;

/// Plugin to load your asset type from MessagePack files.
pub struct MsgPackAssetPlugin<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

impl<A> Plugin for MsgPackAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.add_asset::<A>()
            .add_asset_loader(MsgPackAssetLoader::<A> {
                extensions: self.extensions.clone(),
                _marker: PhantomData,
            });
    }
}

impl<A> MsgPackAssetPlugin<A>
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

struct MsgPackAssetLoader<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

impl<A> AssetLoader for MsgPackAssetLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let asset = from_slice::<A>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}
