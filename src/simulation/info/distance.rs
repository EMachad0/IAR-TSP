use bevy::prelude::*;

use crate::simulation::coord::Coord;
use crate::simulation::graph::city::{Cities, City};

#[derive(Debug, Default)]
pub struct DistanceInfo {
    pub best: f32,
    pub current: f32,
}

pub fn distance_update(
    mut tracker: ResMut<DistanceInfo>,
    cities: Option<Res<Cities>>,
    query: Query<&Coord, With<City>>,
) {
    let cities = match cities {
        None => return,
        Some(cities) => cities,
    };

    let mut distance = 0.0;

    let len = cities.len();
    for i in 0..len {
        let j = (i + 1) % len;

        let u = query.get(cities[i]).unwrap();
        let v = query.get(cities[j]).unwrap();

        distance += u.pos.distance(v.pos)
    }

    tracker.current = distance;
    tracker.best = tracker.best.max(distance);
}
