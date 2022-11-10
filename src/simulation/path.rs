use bevy::prelude::*;
use bevy_prototype_lyon::entity::Path as LyonPath;
use bevy_prototype_lyon::path::ShapePath;
use bevy_prototype_lyon::shapes;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::dataset::{Dataset, DatasetHandle};
use crate::simulation::city::{Cities, City};
use crate::simulation::road::{Road, Roads};

#[derive(Debug)]
pub struct Path {
    best: Vec<usize>,
    current: Vec<usize>,
}

impl Path {
    pub fn random(len: usize) -> Self {
        let mut perm: Vec<usize> = (0..len).collect();
        perm.shuffle(&mut thread_rng());
        Self {
            best: perm.clone(),
            current: perm,
        }
    }
}

pub fn path_setup_on_dataset_load(
    assets: ResMut<Assets<Dataset>>,
    dataset_handle: Res<DatasetHandle>,
    mut commands: Commands,
) {
    let dataset = assets.get(&dataset_handle.handle).unwrap();
    let path = Path::random(dataset.len());
    commands.insert_resource(path);
}

pub fn best_path_update(
    path: Res<Path>,
    roads: Res<Roads>,
    cities: Res<Cities>,
    mut road_query: Query<&mut LyonPath, With<Road>>,
    city_query: Query<&Transform, With<City>>,
) {
    let len = cities.len();
    for i in 0..len {
        let j = (i + 1) % len;

        let u = path.best[i];
        let v = path.best[j];

        let u_pos = city_query.get(cities[u]).unwrap().translation.truncate();
        let v_pos = city_query.get(cities[v]).unwrap().translation.truncate();

        let shape = shapes::Line(u_pos, v_pos);

        let mut road = road_query.get_mut(roads[i]).unwrap();
        *road = ShapePath::build_as(&shape);
    }
}
