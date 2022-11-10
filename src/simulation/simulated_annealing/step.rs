use bevy::prelude::*;
use rand::Rng;

use crate::simulation::coord::Coord;
use crate::simulation::graph::city::{Cities, City};
use crate::simulation::graph::path::Path;
use crate::simulation::info::distance::{compute_distance, DistanceInfo};
use crate::simulation::simulated_annealing::temperature::Temperature;

pub fn simulated_annealing_update(
    mut path: ResMut<Path>,
    cities: Res<Cities>,
    coords: Query<&Coord, With<City>>,
    mut tracker: ResMut<DistanceInfo>,
    temperature: Res<Temperature>,
) {
    let current_distance = tracker.current;

    let neighbour = path.random_neighbour();
    let neighbour_distance = compute_distance(&neighbour, &*cities, &coords);

    let delta = neighbour_distance - current_distance;
    if delta < 0.0 {
        *path = neighbour;
        tracker.current = neighbour_distance;
        if neighbour_distance < tracker.best {
            tracker.best = neighbour_distance;
        }
    } else {
        let p = (-delta / **temperature).exp();
        let mut rng = rand::thread_rng();
        if !p.is_nan() && rng.gen_bool(p as f64) {
            *path = neighbour;
            tracker.current = neighbour_distance;
        }
    }
}