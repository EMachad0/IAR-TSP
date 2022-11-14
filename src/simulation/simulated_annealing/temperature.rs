use bevy::prelude::*;

use crate::consts::{ENDING_TEMPERATURE, ITERATIONS, STARTING_TEMPERATURE};
use crate::simulation::info::update_count::UpdateCountInfo;

#[derive(Debug, Deref, DerefMut)]
pub struct Temperature {
    pub temp: f32,
}

impl Temperature {
    pub fn new(temperature: f32) -> Self {
        Self { temp: temperature }
    }
}

pub fn temperature_update(
    mut temperature: ResMut<Temperature>,
    update_count: Res<UpdateCountInfo>,
) {
    let n = ITERATIONS as f32;
    let i = update_count.update_count as f32;
    let t0 = STARTING_TEMPERATURE;
    let tn = ENDING_TEMPERATURE;

    // let a = ((t0 - tn) * (n + 1.0)) / n;
    // let b = t0 - a;
    // let temp = (a / (i + 1.0)) + b;

    let temp = t0 - i * (t0 - tn) / n;

    let temp = temp.max(ENDING_TEMPERATURE);
    temperature.temp = temp;
}
