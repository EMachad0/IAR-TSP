use bevy::diagnostic::Diagnostics;
use bevy::prelude::*;
use bevy_egui::egui::plot::{Line, Plot, PlotPoints};
use bevy_egui::{egui, EguiContext};

use crate::diagnostics::distance_diagnostic::DistanceDiagnosticsPlugin;
use crate::simulation::control::SimulationStatus;
use crate::simulation::info::update_count::UpdateCountInfo;
use crate::ui::occupied_screen_space::OccupiedScreenSpace;

pub fn side_panel_setup(
    mut egui_ctx: ResMut<EguiContext>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    diagnostics: Res<Diagnostics>,
    mut status: ResMut<SimulationStatus>,
    update_count: Res<UpdateCountInfo>,
) {
    occupied_screen_space.left = egui::SidePanel::left("side_panel")
        .default_width(300.0)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Simulated Annealing Info");

            if let Some(diagnostic) = diagnostics.get(DistanceDiagnosticsPlugin::DISTANCE) {
                if diagnostic.history_len() != 0 {
                    let min = diagnostic.values().map(|f| *f).reduce(f64::min).unwrap();

                    ui.label("Distance: ");
                    ui.horizontal(|ui| {
                        ui.label("Current: ");
                        ui.label(format!("{:.0}", diagnostic.value().unwrap()));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Best: ");
                        ui.label(format!("{:.0}", min));
                    });

                    let distance_over_time: PlotPoints = diagnostic
                        .values()
                        .enumerate()
                        .map(|(i, v)| [i as f64, *v])
                        .collect();

                    Plot::new("distance_plot")
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| plot_ui.line(Line::new(distance_over_time)));

                    ui.separator();
                }
            }
            ui.heading("Temperature:");
            ui.separator();

            ui.heading("Simulation Info");
            ui.horizontal(|ui| {
                ui.label("Update Count: ");
                ui.label(format!("{:.0}", update_count.update_count));
            });
            ui.checkbox(&mut status.paused, "Paused");

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
