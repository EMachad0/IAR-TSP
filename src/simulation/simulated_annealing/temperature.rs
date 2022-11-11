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
    let i = update_count.update_count as f32;
    let temp =
        STARTING_TEMPERATURE - i * (STARTING_TEMPERATURE - ENDING_TEMPERATURE) / ITERATIONS as f32;
    let temp = temp.max(ENDING_TEMPERATURE);
    temperature.temp = temp;
}
