use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy::prelude::*;

use crate::simulation::city::{Cities, City};

#[derive(Default)]
pub struct DistanceDiagnosticsPlugin;

impl Plugin for DistanceDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup)
            .add_system(Self::diagnostic);
    }
}

impl DistanceDiagnosticsPlugin {
    pub const DISTANCE: DiagnosticId =
        DiagnosticId::from_u128(329646547152197190680648809532654742098);

    pub fn setup(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(Self::DISTANCE, "distance", 1));
    }

    pub fn diagnostic(
        mut diagnostics: ResMut<Diagnostics>,
        cities: Option<Res<Cities>>,
        query: Query<&Transform, With<City>>,
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

            let u_pos = u.translation.truncate();
            let v_pos = v.translation.truncate();

            distance += u_pos.distance(v_pos) as f64;
        }

        diagnostics.add_measurement(Self::DISTANCE, || distance);
    }
}
