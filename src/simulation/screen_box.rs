use bevy::prelude::*;

use crate::ui::OccupiedScreenSpace;

#[derive(Debug, Default)]
pub struct SimulationBox {
    pub border: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

impl SimulationBox {
    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    pub fn height(&self) -> f32 {
        self.top - self.bottom
    }

    pub fn translate(&self, point: [f32; 2]) -> [f32; 2] {
        let border_x = self.width() * self.border / 2.;
        let border_y = self.height() * self.border / 2.;

        let [mut x, mut y] = point;

        x = self.left + border_x + x * self.width() * (1. - self.border);
        y = self.bottom + border_y + y * self.height() * (1. - self.border);

        [x, y]
    }
}

pub fn simulation_box_update(
    mut screen: ResMut<SimulationBox>,
    windows: Res<Windows>,
    ui_space: Res<OccupiedScreenSpace>,
) {
    if let Some(window) = windows.get_primary() {
        screen.left = ui_space.left;
        screen.right = window.width() - ui_space.right;
        screen.bottom = ui_space.bottom;
        screen.top = window.height() - ui_space.top;
    }
}
