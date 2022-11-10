use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::simulation::distance::DistanceTracker;

#[derive(Debug, Default)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

pub fn ui_setup(
    mut egui_ctx: ResMut<EguiContext>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    distance_tracker: Res<DistanceTracker>,
) {
    occupied_screen_space.left = egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Side Panel");
            ui.horizontal(|ui| {
                ui.label("Distance: ");
                ui.label(format!("{:?}", distance_tracker.best))
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
