use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::simulation::info::distance::DistanceInfo;
use crate::ui::occupied_screen_space::OccupiedScreenSpace;

pub fn side_panel_setup(
    mut egui_ctx: ResMut<EguiContext>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    distance_tracker: Res<DistanceInfo>,
) {
    occupied_screen_space.left = egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Simulation Info");
            ui.label("Distance: ");
            ui.horizontal(|ui| {
                ui.label("Current: ");
                ui.label(format!("{:?}", distance_tracker.current))
            });
            ui.horizontal(|ui| {
                ui.label("Best: ");
                ui.label(format!("{:?}", distance_tracker.best))
            });
            ui.separator();
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
