use bevy::diagnostic::Diagnostics;
use bevy::prelude::*;

use crate::consts::STARTING_TEMPERATURE;
use crate::diagnostics::distance_diagnostic::DistanceDiagnosticsPlugin;
use crate::diagnostics::temperature_diagnostic::TemperatureDiagnosticsPlugin;
use crate::simulation::coord::Coord;
use crate::simulation::graph::city::{Cities, City};
use crate::simulation::graph::path::Path;
use crate::simulation::info::distance::{compute_distance, DistanceInfo};
use crate::simulation::info::update_count::UpdateCountInfo;
use crate::simulation::simulated_annealing::temperature::Temperature;

#[derive(Debug, Default)]
pub struct ResetControl {
    pub waiting_reset: bool,
}

pub fn is_waiting_reset(control: Res<ResetControl>) -> bool {
    control.waiting_reset
}

pub fn simulation_reset(
    cities: Res<Cities>,
    mut control: ResMut<ResetControl>,
    mut path: ResMut<Path>,
    mut distance: ResMut<DistanceInfo>,
    mut temp: ResMut<Temperature>,
    mut info: ResMut<UpdateCountInfo>,
    mut diagnostics: ResMut<Diagnostics>,
    coords: Query<&Coord, With<City>>,
) {
    *path = Path::random(cities.len());
    distance.current = compute_distance(&path, &cities, &coords);
    temp.temp = STARTING_TEMPERATURE;
    info.update_count = 0;
    if let Some(diagnostic) = diagnostics.get_mut(TemperatureDiagnosticsPlugin::TEMPERATURE) {
        diagnostic.clear_history();
    }
    if let Some(diagnostic) = diagnostics.get_mut(DistanceDiagnosticsPlugin::DISTANCE) {
        diagnostic.clear_history();
    }
    control.waiting_reset = false;
}
