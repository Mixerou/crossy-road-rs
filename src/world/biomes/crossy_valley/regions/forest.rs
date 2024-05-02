use bevy::app::{App, Plugin};
use bevy::ecs::system::SystemId;
use bevy::prelude::{Commands, In, Res, ResMut, World};

use crate::constants::{MAP_GAMEPLAY_MAX_Z, MAP_GAMEPLAY_MIN_Z, MAP_MAX_Z, MAP_MIN_Z};
use crate::resources::grounds::GroundCollection;
use crate::resources::obstacles::ObstacleCollection;
use crate::world::biomes::{spawn_ground, spawn_obstacle};
use crate::world::{Chunk, Map};

pub(in super::super) struct GrassRegion;

impl Plugin for GrassRegion {
    fn build(&self, _: &mut App) {}
}

pub(in super::super) struct SystemIds {
    pub spawn_ground: SystemId<i32>,
    pub spawn_obstacles: SystemId<i32>,
}

impl SystemIds {
    pub fn register(world: &mut World) -> Self {
        Self {
            spawn_ground: world.register_system(Self::spawn_ground),
            spawn_obstacles: world.register_system(Self::spawn_obstacles),
        }
    }

    pub fn unregister(&self, world: &mut World) {
        world.remove_system(self.spawn_ground).unwrap();
        world.remove_system(self.spawn_obstacles).unwrap();
    }

    fn spawn_ground(
        In(x): In<i32>,
        mut commands: Commands,
        mut map: ResMut<Map>,
        grounds: Res<GroundCollection>,
    ) {
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
            let ground = spawn_ground(&mut commands, cube, x, z);

            entities.push(ground);
        }

        map.chunks.push_back(Chunk {
            position_x: x,
            entities,
        });
    }

    fn spawn_obstacles(
        In(x): In<i32>,
        mut commands: Commands,
        mut map: ResMut<Map>,
        obstacles: Res<ObstacleCollection>,
    ) {
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
                    map.random_generator.rand_range(0..4) as f32 / 2.,
                ),
                number if number <= 21 => (
                    &obstacles.stump,
                    map.random_generator.rand_range(0..4) as f32 / 2.,
                ),
                _ => continue,
            };

            spawn_obstacle(&mut commands, &mut map, model, x, z, rotation_factor);
        }
    }
}
