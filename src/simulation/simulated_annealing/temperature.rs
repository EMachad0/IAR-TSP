use bevy::prelude::*;

use crate::consts::MIN_TEMP;
use crate::simulation::control::SimulationStatus;

#[derive(Debug, Deref, DerefMut)]
pub struct Temperature {
    pub temp: f32,
}

impl Temperature {
    pub fn new(temperature: f32) -> Self {
        Self { temp: temperature }
    }
}

pub fn temperature_update(mut temperature: ResMut<Temperature>) {
    temperature.temp /= 2.0;
}

pub fn pause_on_low_temp(mut status: ResMut<SimulationStatus>, info: Res<Temperature>) {
    if info.temp < MIN_TEMP {
        status.paused = true;
    }
}
