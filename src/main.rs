mod camera;
mod consts;
mod dataset;
mod diagnostics;
mod game_state;
mod simulation;
mod timestep;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::prelude::*;
use bevy_prototype_lyon::prelude::*;
use iyes_loopless::prelude::*;
use std::time::Duration;

use crate::consts::*;
use crate::game_state::GameState;

fn main() {
    let mut app = App::new();

    // Resources
    app.insert_resource(ClearColor(Color::WHITE))
        .insert_resource(WindowDescriptor {
            title: "IAR TSP".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .init_resource::<ui::OccupiedScreenSpace>()
        .init_resource::<simulation::info::distance::DistanceInfo>()
        .init_resource::<simulation::info::update_count::UpdateCountInfo>()
        .init_resource::<simulation::control::SimulationStatus>()
        .insert_resource(ui::screen_box::SimulationBox::bordered(0.1))
        .insert_resource(
            simulation::simulated_annealing::temperature::Temperature::new(STARTING_TEMPERATURE),
        );

    // Types
    app.register_type::<simulation::graph::city::City>();

    // Events

    // Stages
    app.add_loopless_state(GameState::Loading).add_stage_before(
        CoreStage::Update,
        timestep::FixedUpdateLabel,
        timestep::FixedTimestepStage::new(Duration::from_secs_f64(STARTING_UPS)),
    );

    // Plugins
    app.add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(dataset::DatasetPlugin)
        .add_plugin(diagnostics::SimulationDiagnosticsPlugin);

    // Setup Systems
    app.add_startup_system(camera::camera_setup);

    // Enter Systems

    // Exit Systems

    // Systems
    app.add_system(ui::side_panel_setup);
    // Loading
    app.add_system_set_to_stage(
        CoreStage::PreUpdate,
        ConditionSet::new()
            .run_in_state(GameState::Loading)
            .run_if(dataset::on_dataset_load)
            .with_system(simulation::graph::path::path_setup_on_dataset_load)
            .with_system(simulation::graph::road::road_setup_on_dataset_load)
            .with_system(simulation::graph::city::city_setup_on_dataset_load)
            .into(),
    );
    app.add_system(game_state::transition_to_simulating.run_if(dataset::on_dataset_load));

    // Simulating
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::Simulating)
            .with_system(simulation::graph::road::road_update)
            .into(),
    )
    .stage(
        timestep::FixedUpdateLabel,
        |stage: &mut timestep::FixedTimestepStage| {
            stage.get_system_stage(1).add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Simulating)
                    .run_if_not(simulation::control::is_simulation_paused)
                    .with_system(simulation::info::update_count::update_count_update)
                    .with_system(simulation::simulated_annealing::step::simulated_annealing_update)
                    .into(),
            );
            stage.get_system_stage(2).add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Simulating)
                    .run_if_not(simulation::control::is_simulation_paused)
                    .with_system(
                        diagnostics::distance_diagnostic::DistanceDiagnosticsPlugin::diagnostic,
                    )
                    .with_system(
                        diagnostics::timestep_diagnostic::TimeStepDiagnosticsPlugin::diagnostic,
                    )
                    .with_system(
                        diagnostics::temperature_diagnostic::TemperatureDiagnosticsPlugin::diagnostic,
                    )
                    .with_system(simulation::simulated_annealing::temperature::temperature_update)
                    .into(),
            );
            stage
                .get_system_stage(2)
                .add_system(simulation::control::auto_pause)
                .add_system(simulation::simulated_annealing::temperature::pause_on_low_temp);
            stage
        },
    );
    app.add_system_set_to_stage(
        CoreStage::PostUpdate,
        ConditionSet::new()
            .run_in_state(GameState::Simulating)
            .with_system(timestep::control::timestep_input_handler)
            .with_system(simulation::control::simulation_pause_input_handler)
            .with_system(ui::screen_box::simulation_box_update)
            .with_system(simulation::graph::city::city_transform_update)
            .into(),
    );

    // Debug
    if cfg!(debug_assertions) {
        app.add_plugin(WorldInspectorPlugin::default());
    }

    // Run
    app.run();
}
