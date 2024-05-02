use bevy::app::{App, Plugin};
use bevy::ecs::system::SystemId;
use bevy::prelude::{Commands, In, Res, ResMut, World};

use crate::constants::{MAP_GAMEPLAY_MAX_Z, MAP_GAMEPLAY_MIN_Z, MAP_MAX_Z, MAP_MIN_Z};
use crate::resources::obstacles::ObstacleCollection;
use crate::world::biomes::spawn_obstacle;
use crate::world::Map;

pub(in super::super) struct SpawnPointRegion;

impl Plugin for SpawnPointRegion {
    fn build(&self, _: &mut App) {}
}

pub(in super::super) struct SystemIds {
    pub spawn_ground: SystemId<i32>,
    pub spawn_obstacles: SystemId<i32>,
}

impl SystemIds {
    pub fn register(world: &mut World, spawn_grass_ground: SystemId<i32>) -> Self {
        Self {
            spawn_ground: spawn_grass_ground,
            spawn_obstacles: world.register_system(Self::spawn_obstacles),
        }
    }

    pub fn unregister(&self, world: &mut World) {
        world.remove_system(self.spawn_obstacles).unwrap();
    }

    fn spawn_obstacles(
        In(x): In<i32>,
        mut commands: Commands,
        mut map: ResMut<Map>,
        obstacles: Res<ObstacleCollection>,
    ) {
        for z in MAP_MIN_Z..MAP_MAX_Z {
            if x > -4 && (MAP_GAMEPLAY_MIN_Z..MAP_GAMEPLAY_MAX_Z + 1).contains(&z) {
                continue;
            }

            let model = obstacles.trees.get_random(&mut map.random_generator);
            let rotation_factor = map.random_generator.rand_range(1..3) as f32;

            spawn_obstacle(&mut commands, &mut map, model, x, z, rotation_factor);
        }
    }
}
