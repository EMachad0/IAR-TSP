mod camera;
mod consts;
mod dataset;
mod diagnostics;
mod game_state;
mod simulation;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::prelude::*;
use bevy_prototype_lyon::prelude::*;
use iyes_loopless::prelude::*;

pub use crate::game_state::GameState;

use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};

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
        .insert_resource(simulation::screen_box::SimulationBox {
            border: 0.1,
            ..default()
        });

    // Types
    app.register_type::<simulation::city::City>();

    // Events

    // Stages
    app.add_loopless_state(GameState::Loading);

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
    app.add_system(ui::ui_setup);
    // Loading
    app.add_system_set_to_stage(
        CoreStage::PreUpdate,
        ConditionSet::new()
            .run_in_state(GameState::Loading)
            .run_if(dataset::on_dataset_load)
            .with_system(simulation::path::path_setup_on_dataset_load)
            .with_system(simulation::road::road_setup_on_dataset_load)
            .with_system(simulation::city::city_setup_on_dataset_load)
            .into(),
    );
    app.add_system(game_state::transition_to_simulating.run_if(dataset::on_dataset_load));

    // Simulating
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::Simulating)
            .with_system(simulation::path::best_path_update)
            .into(),
    );
    app.add_system_set_to_stage(
        CoreStage::PostUpdate,
        ConditionSet::new()
            .run_in_state(GameState::Simulating)
            .with_system(simulation::screen_box::simulation_box_update)
            .with_system(simulation::city::city_transform_update)
            .into(),
    );

    // Debug
    if cfg!(debug_assertions) {
        app.add_plugin(WorldInspectorPlugin::default());
    }

    // Run
    app.run();
}

pub fn create_line(mut commands: Commands) {
    let shape = shapes::Line(Vec2::new(50.0, 50.0), Vec2::new(100.0, 100.0));

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Stroke(StrokeMode::new(Color::GRAY, 5.0)),
        Transform::default(),
    ));
}
