use bevy::prelude::*;
use bevy::render::camera::WindowOrigin;

pub fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            window_origin: WindowOrigin::BottomLeft,
            ..default()
        },
        ..default()
    });
}
