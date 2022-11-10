mod dataset;
mod dataset_error;
mod dataset_loader;

use bevy::prelude::*;

use crate::consts::DATASET_PATH;
pub use crate::dataset::dataset::Dataset;
use crate::dataset::dataset_loader::DatasetAssetLoader;

pub struct DatasetPlugin;

impl Plugin for DatasetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Dataset>()
            .init_asset_loader::<DatasetAssetLoader>()
            .init_resource::<DatasetHandle>()
            .add_startup_system(load_dataset);
    }
}

#[derive(Debug, Default, Deref)]
pub struct DatasetHandle {
    pub handle: Handle<Dataset>,
}

impl DatasetHandle {
    pub fn new(handle: Handle<Dataset>) -> Self {
        Self { handle }
    }
}

pub fn load_dataset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load(DATASET_PATH);
    commands.insert_resource(DatasetHandle::new(handle));
}

pub fn on_dataset_load(
    mut ev_asset: EventReader<AssetEvent<Dataset>>,
    dataset_handle: Res<DatasetHandle>,
) -> bool {
    for ev in ev_asset.iter() {
        if let AssetEvent::Created { handle } = ev {
            if *handle == **dataset_handle {
                return true;
            }
        }
    }
    false
}
