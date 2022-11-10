use bevy::prelude::*;

#[derive(Debug, Deref, DerefMut)]
pub struct Temperature {
    t: f32,
}

impl Temperature {
    pub fn new(temperature: f32) -> Self {
        Self { t: temperature }
    }
}

pub fn temperature_update(mut temperature: ResMut<Temperature>) {
    temperature.t /= 2.0;
}
