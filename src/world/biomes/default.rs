use bevy::app::{App, Plugin, Update};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec3;
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle, PbrBundle};
use bevy::prelude::{
    in_state, Color, Commands, Entity, EventReader, IntoSystemConfigs, NextState, OnEnter, OnExit,
    Query, Res, ResMut, Transform, With,
};
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::Collider;

use crate::constants::{GAMEPLAY_MAX_Z, GAMEPLAY_MIN_Z};
use crate::events::RequestChunkGeneration;
use crate::resources::ObjectCollection;
use crate::states::CurrentBiome;
use crate::world::{Chunk, Map};

pub struct DefaultBiome;

impl Plugin for DefaultBiome {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CurrentBiome::Default), Self::enter_the_biome)
            .add_systems(
                Update,
                Self::spawn_chunk.run_if(in_state(CurrentBiome::Default)),
            )
            .add_systems(OnExit(CurrentBiome::Default), Self::leave_the_biome);
    }
}

impl DefaultBiome {
    fn enter_the_biome(mut commands: Commands) {
        commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 7_000.,
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.1, 5.5, -3.))
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

    fn spawn_chunk(
        mut commands: Commands,
        mut map: ResMut<Map>,
        objects: Res<ObjectCollection>,
        mut chunk_generation_requests: EventReader<RequestChunkGeneration>,
    ) {
        for _ in chunk_generation_requests.read() {
            let mut entities = Vec::new();
            let x = match map.chunks.len() {
                0 => -8,
                _ => match map.chunks.back() {
                    Some(chunk) => chunk.position_x + 1,
                    None => -8,
                },
            };

            for z in -10..10 {
                let cube = if x % 2 == 0 {
                    match (GAMEPLAY_MIN_Z..=GAMEPLAY_MAX_Z).contains(&z) {
                        true => objects.light_cube.clone(),
                        false => objects.light_dimmed_block.clone(),
                    }
                } else {
                    match (GAMEPLAY_MIN_Z..=GAMEPLAY_MAX_Z).contains(&z) {
                        true => objects.dark_cube.clone(),
                        false => objects.dark_dimmed_cube.clone(),
                    }
                };

                let entity = commands.spawn((
                    PbrBundle {
                        mesh: cube.mesh,
                        material: cube.material,
                        transform: Transform::from_xyz(x as f32, 0., z as f32),
                        ..Default::default()
                    },
                    RigidBody::Fixed,
                    Collider::cuboid(0.5, 0.5, 0.5),
                ));

                entities.push(entity.id());
            }

            map.chunks.push_back(Chunk {
                position_x: x,
                entities,
            });
        }
    }

    fn leave_the_biome(
        mut commands: Commands,
        mut current_biome: ResMut<NextState<CurrentBiome>>,
        mut map: ResMut<Map>,
        lights: Query<Entity, With<DirectionalLight>>,
    ) {
        current_biome.set(CurrentBiome::None);

        for entity in map.chunks.iter().flat_map(|chunk| &chunk.entities) {
            if let Some(entity) = commands.get_entity(*entity) {
                entity.despawn_recursive();
            }
        }

        map.chunks.clear();

        for entity in &lights {
            if let Some(entity) = commands.get_entity(entity) {
                entity.despawn_recursive();
            }
        }
    }
}
