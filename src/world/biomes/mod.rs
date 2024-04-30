use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec3;
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle};
use bevy::prelude::{
    Color, Commands, Component, Entity, EventReader, Or, Query, ResMut, Transform, With,
};

use crate::constants::{MAP_MAX_Z, MAP_MIN_Z};
use crate::events::RequestOldChunkDespawning;
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
                first_cascade_far_bound: 7.0,
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
                if let Some(entity) = map.obstacles_xz.remove(&(chunk_position_x, z)) {
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

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Obstacle;
