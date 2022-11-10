use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

pub mod distance_diagnostic;

pub struct SimulationDiagnosticsPlugin;

impl Plugin for SimulationDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(distance_diagnostic::DistanceDiagnosticsPlugin);
    }
}
