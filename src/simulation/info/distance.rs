use bevy::prelude::*;

use crate::simulation::coord::Coord;
use crate::simulation::graph::city::{Cities, City};
use crate::simulation::graph::path::Path;

#[derive(Debug)]
pub struct DistanceInfo {
    pub best: f32,
    pub current: f32,
}

impl Default for DistanceInfo {
    fn default() -> Self {
        Self {
            best: f32::MAX,
            current: f32::MAX,
        }
    }
}

pub fn compute_distance(path: &Path, cities: &Cities, coords: &Query<&Coord, With<City>>) -> f32 {
    let len = path.len();

    let mut distance = 0.0;
    for i in 0..len {
        let j = (i + 1) % len;

        let u = coords.get(cities[path[i]]).unwrap();
        let v = coords.get(cities[path[j]]).unwrap();

        distance += u.pos.distance(v.pos)
    }
    distance
}
