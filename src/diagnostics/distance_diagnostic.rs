use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy::prelude::*;

use crate::consts::PLOT_ITERATIONS;
use crate::simulation::info::distance::DistanceInfo;

#[derive(Default)]
pub struct DistanceDiagnosticsPlugin;

impl Plugin for DistanceDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup);
    }
}

impl DistanceDiagnosticsPlugin {
    pub const DISTANCE: DiagnosticId =
        DiagnosticId::from_u128(329646547152197190680648809532654742098);

    pub fn setup(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(Self::DISTANCE, "distance", PLOT_ITERATIONS));
    }

    pub fn diagnostic(mut diagnostics: ResMut<Diagnostics>, tracker: Res<DistanceInfo>) {
        diagnostics.add_measurement(Self::DISTANCE, || tracker.current as f64);
    }
}
