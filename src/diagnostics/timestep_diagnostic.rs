use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy::prelude::*;
use std::time::Duration;

use crate::timestep::FixedTimestepInfo;

/// Adds "fixed_timestep" diagnostic to an App, specifically "step time", "sps", "step count", "overstep" and "accumulator"
#[derive(Default)]
pub struct TimeStepDiagnosticsPlugin;

pub struct TimeStepDiagnosticsState {
    pub(crate) update_count: u64,
}

impl Plugin for TimeStepDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup)
            .insert_resource(TimeStepDiagnosticsState { update_count: 0 });
    }
}

impl TimeStepDiagnosticsPlugin {
    pub const SPS: DiagnosticId = DiagnosticId::from_u128(28814478231947238174408528866909494);
    pub const STEP_COUNT: DiagnosticId =
        DiagnosticId::from_u128(54021991829115352065418785002088010288);
    pub const STEP_TIME: DiagnosticId =
        DiagnosticId::from_u128(73441630925388532774622109383099159600);
    pub const OVERSTEP: DiagnosticId =
        DiagnosticId::from_u128(74441630414314879147813471899499159600);
    pub const ACCUMULATOR: DiagnosticId =
        DiagnosticId::from_u128(75441630925388532471389418383099159600);

    pub fn setup(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(Self::STEP_TIME, "step_time", 20).with_suffix("s"));
        diagnostics.add(Diagnostic::new(Self::SPS, "sps", 1));
        diagnostics.add(Diagnostic::new(Self::STEP_COUNT, "step_count", 1));
        diagnostics.add(Diagnostic::new(Self::OVERSTEP, "overstep", 20));
        diagnostics.add(Diagnostic::new(Self::ACCUMULATOR, "accumulator", 20).with_suffix("s"));
    }

    pub fn diagnostic(
        mut diagnostics: ResMut<Diagnostics>,
        mut state: ResMut<TimeStepDiagnosticsState>,
        info: Res<FixedTimestepInfo>,
    ) {
        let FixedTimestepInfo { step, accumulator } = *info;

        diagnostics.add_measurement(TimeStepDiagnosticsPlugin::STEP_COUNT, || {
            state.update_count = state.update_count.wrapping_add(1);
            state.update_count as f64
        });
        diagnostics.add_measurement(TimeStepDiagnosticsPlugin::STEP_TIME, || step.as_secs_f64());
        diagnostics.add_measurement(TimeStepDiagnosticsPlugin::ACCUMULATOR, || {
            accumulator.as_secs_f64()
        });
        if step > Duration::ZERO {
            diagnostics
                .add_measurement(TimeStepDiagnosticsPlugin::SPS, || 1.0 / step.as_secs_f64());
            diagnostics.add_measurement(TimeStepDiagnosticsPlugin::OVERSTEP, || {
                accumulator.as_secs_f64() / step.as_secs_f64()
            });
        }
    }
}
