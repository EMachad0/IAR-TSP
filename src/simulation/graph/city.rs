use bevy::prelude::*;

use crate::dataset::Dataset;
use crate::simulation::coord::Coord;
use crate::ui::screen_box::SimulationBox;

#[derive(Default, Deref, DerefMut)]
pub struct Cities {
    vec: Vec<Entity>,
}

#[derive(Debug, Default, Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct City {
    x: f32,
    y: f32,
}

impl From<[f32; 2]> for City {
    fn from([x, y]: [f32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<City> for [f32; 2] {
    fn from(City { x, y }: City) -> Self {
        [x, y]
    }
}

pub fn city_setup_on_dataset_load(
    mut ev_asset: EventReader<AssetEvent<Dataset>>,
    assets: ResMut<Assets<Dataset>>,
    mut commands: Commands,
    screen: Res<SimulationBox>,
) {
    for ev in ev_asset.iter() {
        if let AssetEvent::Created { handle } = ev {
            let dataset = assets.get(handle).unwrap();
            let data = &dataset.data;

            if data.is_empty() {
                continue;
            }

            let min_x = data.iter().map(|[x, _]| *x).reduce(f32::min).unwrap();
            let max_x = data.iter().map(|[x, _]| *x).reduce(f32::max).unwrap();
            let min_y = data.iter().map(|[_, y]| *y).reduce(f32::min).unwrap();
            let max_y = data.iter().map(|[_, y]| *y).reduce(f32::max).unwrap();

            let mut city_entities = Vec::with_capacity(data.len());

            for (i, coord) in data.iter().enumerate() {
                let [x, y] = coord;
                let position = [(x - min_x) / (max_x - min_x), (y - min_y) / (max_y - min_y)];
                let screen_position = screen.translate(position);

                let entity = commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(10.0, 10.0)),
                            color: Color::BLACK,
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::from((
                            screen_position.into(),
                            0.0,
                        ))),
                        ..default()
                    })
                    .insert(City::from(position))
                    .insert(Name::new(format!("City {}", i)))
                    .insert(Coord::new(Vec2::from(*coord)))
                    .id();

                city_entities.push(entity)
            }

            commands.insert_resource(Cities { vec: city_entities });
        }
    }
}

pub fn city_transform_update(
    screen: Res<SimulationBox>,
    mut query: Query<(&mut Transform, &City)>,
) {
    for (mut transform, city) in query.iter_mut() {
        let screen_position = screen.translate((*city).into());
        transform.translation = Vec3::from((screen_position.into(), 0.0));
    }
}
