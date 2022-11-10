use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::simulation::city::City;

const BORDER_PERCENTAGE: f32 = 0.1;

pub fn to_screen_position(width: f32, height: f32, point: &[f32; 2]) -> [f32; 2] {
    let border_x = width * BORDER_PERCENTAGE / 2.;
    let border_y = height * BORDER_PERCENTAGE / 2.;

    let [mut x, mut y] = point;

    x = border_x + x * width * (1. - BORDER_PERCENTAGE);
    y = border_y + y * height * (1. - BORDER_PERCENTAGE);

    [x, y]
}

pub fn transform_update_on_resize(
    mut ev_window: EventReader<WindowResized>,
    mut query: Query<(&City, &mut Transform)>,
) {
    if let Some(ev) = ev_window.iter().last() {
        for (city, mut transform) in query.iter_mut() {
            let point: [f32; 2] = (*city).into();
            let pos = to_screen_position(ev.width, ev.height, &point);
            transform.translation = Vec3::from((pos.into(), 0.0));
        }
    }
}
