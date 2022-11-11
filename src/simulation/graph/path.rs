use bevy::prelude::*;
use rand::distributions::Uniform;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::dataset::{Dataset, DatasetHandle};

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Path {
    path: Vec<usize>,
}

#[derive(Debug)]
pub struct BestPath {
    pub path: Path,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PathType {
    CURRENT,
    BEST,
}

impl std::fmt::Display for PathType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PathType::CURRENT => write!(f, "Current"),
            PathType::BEST => write!(f, "Best"),
        }
    }
}

impl Path {
    pub fn random(len: usize) -> Self {
        let mut perm: Vec<usize> = (0..len).collect();
        perm.shuffle(&mut thread_rng());
        Self { path: perm }
    }

    pub fn random_neighbour(&self) -> Self {
        let mut rng = thread_rng();
        let mut path = self.path.clone();
        let uniform = Uniform::from(0..self.len());
        path.swap(rng.sample(uniform), rng.sample(uniform));
        Self { path }
    }
}

pub fn path_setup_on_dataset_load(
    assets: ResMut<Assets<Dataset>>,
    dataset_handle: Res<DatasetHandle>,
    mut commands: Commands,
) {
    let dataset = assets.get(&dataset_handle.handle).unwrap();
    let path = Path::random(dataset.len());
    commands.insert_resource(path.clone());
    commands.insert_resource(BestPath { path });
}
