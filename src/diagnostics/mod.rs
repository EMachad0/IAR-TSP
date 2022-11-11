use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

pub mod distance_diagnostic;
pub mod temperature_diagnostic;
pub mod timestep_diagnostic;

pub struct SimulationDiagnosticsPlugin;

impl Plugin for SimulationDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(timestep_diagnostic::TimeStepDiagnosticsPlugin)
            .add_plugin(distance_diagnostic::DistanceDiagnosticsPlugin)
            .add_plugin(temperature_diagnostic::TemperatureDiagnosticsPlugin);
    }
}
