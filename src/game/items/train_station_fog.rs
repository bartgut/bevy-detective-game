use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::clickable::items::resource::ItemResource;
use crate::level_state::LevelState;
use crate::materials::fog::FogMaterial;
use crate::spawnable::components::{Spawnable, SpawnableChild};

#[derive(Component)]
pub struct TrainStationFog {
    pub material: Handle<FogMaterial>,
    pub mesh: Mesh2dHandle,
}

pub fn train_station_fog_prepare(
    mut items: ResMut<ItemResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FogMaterial>>,
) {
    let mesh = meshes
        .add(Mesh::from(shape::Quad {
            size: Vec2::new(2560.0, 450.0),
            ..default()
        }))
        .into();

    let material = materials.add(FogMaterial {
        fog_size: Vec2::new(1.0, 1.0),
        fog_color: Color::rgba(1.0, 1.0, 1.0, 0.22),
        fog_speed: Vec2::new(0.05, 0.0),
    });

    items
        .items
        .entry(LevelState::TrainPlatform)
        .and_modify(|items| items.push(Box::new(TrainStationFog { material, mesh })));
}

impl SpawnableChild for TrainStationFog {
    fn spawn_child(&self, level: &mut EntityCommands, _: &Res<AssetServer>) {
        level.with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: self.mesh.clone(),
                transform: Transform::from_xyz(0.0, 0.5, 100.0),
                material: self.material.clone_weak(),
                ..default()
            });
        });
    }
}
