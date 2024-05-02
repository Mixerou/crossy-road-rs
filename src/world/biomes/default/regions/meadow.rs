use bevy::app::{App, Plugin};
use bevy::ecs::system::SystemId;
use bevy::prelude::{Commands, In, Res, ResMut, World};

use crate::constants::{MAP_GAMEPLAY_MAX_Z, MAP_GAMEPLAY_MIN_Z, MAP_MAX_Z, MAP_MIN_Z};
use crate::resources::grounds::GroundCollection;
use crate::world::biomes::spawn_ground;
use crate::world::{Chunk, Map};

/// Plugin containing region-specific systems, resources, etc.
pub(in super::super) struct GrassRegion;

impl Plugin for GrassRegion {
    fn build(&self, _: &mut App) {}
}

/// Systems that are invoked from the `spawn_new_chunk` biome system.
pub(in super::super) struct SystemIds {
    pub spawn_ground: SystemId<i32>,
    pub spawn_obstacles: SystemId<i32>,
}

impl SystemIds {
    /// Normally called when entering the biome
    /// and then stores these ids in BiomeData `struct`, for example.
    pub fn register(world: &mut World) -> Self {
        Self {
            spawn_ground: world.register_system(Self::spawn_ground),
            spawn_obstacles: world.register_system(Self::spawn_obstacles),
        }
    }

    /// Normally called when leaving the biome.
    pub fn unregister(&self, world: &mut World) {
        world.remove_system(self.spawn_ground).unwrap();
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

    fn spawn_obstacles(In(_x): In<i32>) {}
}
