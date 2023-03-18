use bevy::app::{App, Plugin};
use bevy::asset::{AddAsset, Asset, AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use quick_xml::de::from_str;
use std::marker::PhantomData;
use std::str::from_utf8;

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
        app.add_asset::<A>().add_asset_loader(XmlAssetLoader::<A> {
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

struct XmlAssetLoader<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

impl<A> AssetLoader for XmlAssetLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let asset = from_str::<A>(from_utf8(bytes)?)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}
