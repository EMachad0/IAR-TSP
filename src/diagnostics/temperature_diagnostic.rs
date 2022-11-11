use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy::prelude::*;

use crate::consts::ITERATIONS;
use crate::simulation::simulated_annealing::temperature::Temperature;

#[derive(Default)]
pub struct TemperatureDiagnosticsPlugin;

impl Plugin for TemperatureDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup);
    }
}

impl TemperatureDiagnosticsPlugin {
    pub const TEMPERATURE: DiagnosticId =
        DiagnosticId::from_u128(73815967321894632935461374174954372577);

    pub fn setup(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(Self::TEMPERATURE, "distance", ITERATIONS));
    }

    pub fn diagnostic(mut diagnostics: ResMut<Diagnostics>, tracker: Res<Temperature>) {
        diagnostics.add_measurement(Self::TEMPERATURE, || tracker.temp as f64);
    }
}
