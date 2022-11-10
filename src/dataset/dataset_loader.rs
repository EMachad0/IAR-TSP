use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};

use crate::dataset::dataset::Dataset;

#[derive(Default)]
pub struct DatasetAssetLoader;

impl AssetLoader for DatasetAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let asset = Dataset::from_buffer(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tsp", "txt"]
    }
}
