use std::f32::consts::PI;

use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::{IVec2, Quat, Vec3};
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle, PbrBundle};
use bevy::prelude::{
    Color, Commands, Component, Entity, EventReader, Or, Query, ResMut, Transform, With,
};
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::Collider;

use crate::constants::{MAP_MAX_Z, MAP_MIN_Z};
use crate::events::RequestOldChunkDespawning;
use crate::resources::Model;
use crate::world::Map;

pub mod crossy_valley;
#[cfg(feature = "debug")]
pub mod default;

struct StandardBiomeSystems;

impl StandardBiomeSystems {
    fn enter_biome(mut commands: Commands) {
        commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 7_000.,
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0., 5.5, -3.))
                .looking_at(Vec3::ZERO, Vec3::Y),
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 7.,
                minimum_distance: 0.,
                maximum_distance: 0.1,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        });

        commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::rgb(219. / 255., 220. / 255., 1.),
                illuminance: 3_000.,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-3., 2., 1.5))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
    }

    fn despawn_old_chunk(
        mut commands: Commands,
        mut old_chunk_despawning_requests: EventReader<RequestOldChunkDespawning>,
        mut map: ResMut<Map>,
    ) {
        for _ in old_chunk_despawning_requests.read() {
            let Some(chunk) = map.chunks.front() else {
                continue;
            };
            let chunk_position_x = chunk.position_x;

            for entity in &chunk.entities {
                if let Some(entity) = commands.get_entity(*entity) {
                    entity.despawn_recursive();
                }
            }

            for z in MAP_MIN_Z..MAP_MAX_Z {
                if let Some(entity) = map.obstacles_xz.remove(&IVec2::new(chunk_position_x, z)) {
                    if let Some(entity) = commands.get_entity(entity) {
                        entity.despawn_recursive();
                    }
                }
            }

            map.chunks.pop_front();
        }
    }

    fn leave_biome(
        mut commands: Commands,
        mut map: ResMut<Map>,
        entities: Query<Entity, Or<(With<DirectionalLight>, With<Ground>, With<Obstacle>)>>,
    ) {
        for entity in &entities {
            if let Some(entity) = commands.get_entity(entity) {
                entity.despawn_recursive();
            }
        }

        map.chunks.clear();
        map.obstacles_xz.clear();
    }
}

fn spawn_ground(commands: &mut Commands, model: &Model, x: i32, z: i32) -> Entity {
    commands
        .spawn((
            PbrBundle {
                mesh: model.mesh.clone_weak(),
                material: model.material.clone_weak(),
                transform: Transform::from_xyz(x as f32, 0., z as f32),
                ..Default::default()
            },
            Ground,
            RigidBody::Fixed,
            Collider::cuboid(
                model.mesh_size.x / 2.,
                model.mesh_size.y / 2.,
                model.mesh_size.z / 2.,
            ),
        ))
        .id()
}

fn spawn_obstacle(
    commands: &mut Commands,
    map: &mut ResMut<Map>,
    model: &Model,
    x: i32,
    z: i32,
    rotation_factor: f32,
) {
    let obstacle = commands.spawn((
        PbrBundle {
            mesh: model.mesh.clone_weak(),
            material: model.material.clone_weak(),
            transform: Transform::from_xyz(x as f32, 0.5 + model.mesh_size.y / 2., z as f32)
                .with_rotation(Quat::from_rotation_y(rotation_factor * PI)),
            ..Default::default()
        },
        Obstacle,
    ));

    map.obstacles_xz.insert(IVec2::new(x, z), obstacle.id());
}

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Obstacle;
