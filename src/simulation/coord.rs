use bevy::prelude::*;

#[derive(Debug, Default, Copy, Clone, Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct Coord {
    pub pos: Vec2,
}

impl Coord {
    pub fn new(pos: Vec2) -> Self {
        Self { pos }
    }
}
