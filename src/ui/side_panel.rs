use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::egui::plot::{Line, Plot, PlotPoints};
use bevy_egui::egui::Ui;
use bevy_egui::{egui, EguiContext};
use std::hash::Hash;

use crate::consts::MAX_PLOT_POINTS;
use crate::diagnostics::distance_diagnostic::DistanceDiagnosticsPlugin;
use crate::diagnostics::temperature_diagnostic::TemperatureDiagnosticsPlugin;
use crate::diagnostics::timestep_diagnostic::TimeStepDiagnosticsPlugin;
use crate::simulation::control::SimulationStatus;
use crate::simulation::graph::path::PathType;
use crate::simulation::graph::road::RoadDisplayedPath;
use crate::simulation::info::distance::DistanceInfo;
use crate::simulation::info::update_count::UpdateCountInfo;
use crate::simulation::reset::ResetControl;
use crate::timestep::{FixedTimestepConfig, FixedTimestepInfo};
use crate::ui::occupied_screen_space::OccupiedScreenSpace;

pub fn side_panel_setup(
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    diagnostics: Res<Diagnostics>,
    displayed_path: Option<ResMut<RoadDisplayedPath>>,
    mut status: ResMut<SimulationStatus>,
    mut reset_control: ResMut<ResetControl>,
    fixed_timestep_info: Option<Res<FixedTimestepInfo>>,
    distance_info: Res<DistanceInfo>,
    update_count: Res<UpdateCountInfo>,
) {
    occupied_screen_space.left = egui::SidePanel::left("side_panel")
        .default_width(300.0)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Simulated Annealing Info");

            ui.heading("Distance: ");
            ui.horizontal(|ui| {
                ui.label("Current: ");
                ui.label(format!("{:.0}", distance_info.current));
            });
            ui.horizontal(|ui| {
                ui.label("Best: ");
                ui.label(format!("{:.0}", distance_info.best));
            });
            if let Some(diagnostic) = diagnostics.get(DistanceDiagnosticsPlugin::DISTANCE) {
                if diagnostic.history_len() != 0 {
                    plot(ui, diagnostic.values(), "distance_plot");
                }
            }
            ui.separator();
            if let Some(diagnostic) = diagnostics.get(TemperatureDiagnosticsPlugin::TEMPERATURE) {
                if diagnostic.history_len() != 0 {
                    ui.heading("Temperature:");
                    ui.horizontal(|ui| {
                        ui.label("Current: ");
                        ui.label(format!("{:.6}", diagnostic.value().unwrap()));
                    });
                    plot(ui, diagnostic.values(), "temperature_plot");
                    ui.separator();
                }
            }
            ui.separator();

            ui.heading("Simulation Info");
            ui.horizontal(|ui| {
                ui.label("Update Count: ");
                ui.label(format!("{:.0}", update_count.update_count));
            });
            ui.checkbox(&mut status.paused, "Paused");
            if let Some(mut displayed_path) = displayed_path {
                ui.horizontal(|ui| {
                    ui.label("Displayed Path: ");
                    ui.selectable_value(
                        &mut displayed_path.0,
                        PathType::CURRENT,
                        PathType::CURRENT.to_string(),
                    );
                    ui.selectable_value(
                        &mut displayed_path.0,
                        PathType::BEST,
                        PathType::BEST.to_string(),
                    );
                });
            }
            if let Some(diagnostics) = diagnostics.get(TimeStepDiagnosticsPlugin::SPS) {
                if let Some(value) = diagnostics.value() {
                    ui.horizontal(|ui| {
                        ui.label("Updates per second: ");
                        if let Some(info) = fixed_timestep_info {
                            if ui.button("-").clicked() {
                                commands.insert_resource(FixedTimestepConfig {
                                    step: Some(info.step * 2),
                                    ..default()
                                });
                            }
                            if ui.button("+").clicked() {
                                commands.insert_resource(FixedTimestepConfig {
                                    step: Some(info.step / 2),
                                    ..default()
                                });
                            }
                        }
                        ui.label(format!("{:.0}", value));
                    });
                }
            }
            if let Some(diagnostics) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = diagnostics.value() {
                    ui.horizontal(|ui| {
                        ui.label("Frames Per Second: ");
                        ui.label(format!("{:.0}", value));
                    });
                }
            }
            if ui.button("Reset").clicked() {
                reset_control.waiting_reset = true;
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}

pub fn plot<'a, I, H>(ui: &mut Ui, itr: I, id: H)
where
    I: IntoIterator<Item = &'a f64>,
    H: Hash,
{
    let points = itr.into_iter().enumerate().collect::<Vec<_>>();
    let chunk_size = (points.len() / MAX_PLOT_POINTS).max(1);

    let over_time: PlotPoints = points
        .chunks(chunk_size)
        .map(|c| *c.last().unwrap())
        .map(|(i, v)| [i as f64, *v])
        .collect();

    Plot::new(id)
        .view_aspect(2.0)
        .show(ui, |plot_ui| plot_ui.line(Line::new(over_time)));
}
