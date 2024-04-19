use std::f32::consts::PI;

use bevy::app::{App, Plugin, Update};
use bevy::math::Quat;
use bevy::pbr::PbrBundle;
use bevy::prelude::{
    in_state, Commands, EventReader, In, IntoSystem, IntoSystemConfigs, OnEnter, OnExit, Res,
    ResMut, Transform,
};
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::Collider;

use crate::constants::{MAP_GAMEPLAY_MAX_Z, MAP_GAMEPLAY_MIN_Z, MAP_MAX_Z, MAP_MIN_X, MAP_MIN_Z};
use crate::events::RequestNewChunkSpawning;
use crate::resources::grounds::GroundCollection;
use crate::resources::obstacles::ObstacleCollection;
use crate::states::CurrentBiome;
use crate::world::biomes::{Ground, Obstacle, StandardBiomeSystems};
use crate::world::{Chunk, Map};

pub struct CrossyValleyBiome;

const CURRENT_BIOME: CurrentBiome = CurrentBiome::CrossyValley;

impl Plugin for CrossyValleyBiome {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_BIOME), StandardBiomeSystems::enter_biome)
            .add_systems(
                Update,
                (
                    Self::spawn_new_chunk
                        .pipe(Self::spawn_ground)
                        .pipe(Self::spawn_obstacles),
                    StandardBiomeSystems::despawn_old_chunk,
                )
                    .distributive_run_if(in_state(CURRENT_BIOME)),
            )
            .add_systems(OnExit(CURRENT_BIOME), StandardBiomeSystems::leave_biome);
    }
}

impl CrossyValleyBiome {
    fn spawn_new_chunk(
        mut new_chunk_spawning_requests: EventReader<RequestNewChunkSpawning>,
        map: Res<Map>,
    ) -> Vec<i32> {
        let from_x = match map.chunks.is_empty() {
            true => MAP_MIN_X,
            false => match map.chunks.back() {
                Some(chunk) => chunk.position_x + 1,
                None => MAP_MIN_X,
            },
        };
        let range = from_x..(from_x + new_chunk_spawning_requests.len() as i32);

        new_chunk_spawning_requests.clear();

        range.collect()
    }

    fn spawn_ground(
        In(x): In<Vec<i32>>,
        mut commands: Commands,
        mut map: ResMut<Map>,
        grounds: Res<GroundCollection>,
    ) -> Vec<i32> {
        for x in x.clone() {
            let mut entities = Vec::new();

            for z in MAP_MIN_Z..MAP_MAX_Z {
                let cube = if x % 2 == 0 {
                    match (MAP_GAMEPLAY_MIN_Z..=MAP_GAMEPLAY_MAX_Z).contains(&z) {
                        true => &grounds.light_cube.default,
                        false => &grounds.light_cube.dimmed,
                    }
                } else {
                    match (MAP_GAMEPLAY_MIN_Z..=MAP_GAMEPLAY_MAX_Z).contains(&z) {
                        true => &grounds.dark_cube.default,
                        false => &grounds.dark_cube.dimmed,
                    }
                };

                let ground = commands.spawn((
                    PbrBundle {
                        mesh: cube.mesh.clone_weak(),
                        material: cube.material.clone_weak(),
                        transform: Transform::from_xyz(x as f32, 0., z as f32),
                        ..Default::default()
                    },
                    Ground,
                    RigidBody::Fixed,
                    Collider::cuboid(0.5, 0.5, 0.5),
                ));

                entities.push(ground.id());
            }

            map.chunks.push_back(Chunk {
                position_x: x,
                entities,
            });
        }

        x
    }

    fn spawn_obstacles(
        In(x): In<Vec<i32>>,
        mut commands: Commands,
        mut map: ResMut<Map>,
        obstacles: Res<ObstacleCollection>,
    ) {
        for x in x {
            for z in MAP_MIN_Z..MAP_MAX_Z {
                if z == 0 {
                    continue;
                }

                let (model, rotation_factor) = match map.random_generator.rand_range(0..101) {
                    number if number <= 15 => (
                        obstacles.trees.get_random(&mut map.random_generator),
                        map.random_generator.rand_range(1..3) as f32,
                    ),
                    number if number <= 20 => (
                        &obstacles.boulder,
                        map.random_generator.rand_range(1..4) as f32 / 2.,
                    ),
                    _ => continue,
                };

                let obstacle = commands.spawn((
                    PbrBundle {
                        mesh: model.mesh.clone_weak(),
                        material: model.material.clone_weak(),
                        transform: Transform::from_xyz(
                            x as f32,
                            0.5 + model.mesh_size.y / 2.,
                            z as f32,
                        )
                        .with_rotation(Quat::from_rotation_y(rotation_factor * PI)),
                        ..Default::default()
                    },
                    Obstacle,
                ));

                map.obstacles_xz.insert((x, z), obstacle.id());
            }
        }
    }
}
