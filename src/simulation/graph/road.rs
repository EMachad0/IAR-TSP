use bevy::prelude::*;
use bevy_prototype_lyon::entity::Path as LyonPath;
use bevy_prototype_lyon::prelude::*;

use crate::dataset::Dataset;
use crate::simulation::graph::city::{Cities, City};
use crate::simulation::graph::path::Path;

#[derive(Default, Deref, DerefMut)]
pub struct Roads {
    vec: Vec<Entity>,
}

#[derive(Debug, Default, Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Road;

pub fn road_setup_on_dataset_load(
    mut ev_asset: EventReader<AssetEvent<Dataset>>,
    assets: ResMut<Assets<Dataset>>,
    mut commands: Commands,
) {
    for ev in ev_asset.iter() {
        if let AssetEvent::Created { handle } = ev {
            let dataset = assets.get(handle).unwrap();

            let mut road_entities = Vec::with_capacity(dataset.len());

            for i in 0..dataset.len() {
                let entity = commands
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Line(Vec2::default(), Vec2::default()),
                        DrawMode::Stroke(StrokeMode::new(Color::GRAY, 5.0)),
                        Transform::default(),
                    ))
                    .insert(Road)
                    .insert(Name::new(format!("Road {}", i)))
                    .id();

                road_entities.push(entity)
            }

            commands.insert_resource(Roads { vec: road_entities });
        }
    }
}

pub fn road_update(
    path: Res<Path>,
    roads: Res<Roads>,
    cities: Res<Cities>,
    mut road_query: Query<&mut LyonPath, With<Road>>,
    city_query: Query<&Transform, With<City>>,
) {
    let len = cities.len();
    for i in 0..len {
        let j = (i + 1) % len;

        let u = path[i];
        let v = path[j];

        let u_pos = city_query.get(cities[u]).unwrap().translation.truncate();
        let v_pos = city_query.get(cities[v]).unwrap().translation.truncate();

        let shape = shapes::Line(u_pos, v_pos);

        let mut road = road_query.get_mut(roads[i]).unwrap();
        *road = ShapePath::build_as(&shape);
    }
}
