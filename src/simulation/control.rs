use bevy::prelude::*;

use crate::simulation::info::update_count::UpdateCountInfo;

#[derive(Debug, Copy, Clone, Default)]
pub struct SimulationStatus {
    pub paused: bool,
}

pub fn is_simulation_paused(status: Res<SimulationStatus>) -> bool {
    status.paused
}

pub fn simulation_pause_input_handler(
    kbd: Res<Input<KeyCode>>,
    mut status: ResMut<SimulationStatus>,
) {
    if kbd.just_pressed(KeyCode::Space) {
        status.paused = !status.paused;
    }
}

pub fn auto_pause(mut status: ResMut<SimulationStatus>, info: Res<UpdateCountInfo>) {
    if info.update_count % 1_000_000 == 0 {
        status.paused = true;
    }
}
