use std::collections::VecDeque;

use bevy::app::{App, Plugin, Update};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec3;
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle, PbrBundle};
use bevy::prelude::{
    in_state, Color, Commands, Entity, IntoSystemConfigs, NextState, OnEnter, Query, Res, ResMut,
    Resource, Transform, ViewVisibility,
};
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::Collider;

use crate::constants::{GAMEPLAY_MAX_Z, GAMEPLAY_MIN_Z};
use crate::resources::ObjectCollection;
use crate::states::AppState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Biome::default())
            .add_systems(
                OnEnter(AppState::InitialisingWorld),
                (init_lights, init_world),
            )
            .add_systems(
                Update,
                check_chunk_visibilities.run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Clone)]
pub struct Chunk {
    pub position_x: isize,
    pub entities: Vec<Entity>,
}

#[derive(Clone, Default, Resource)]
pub struct Biome {
    pub chunks: VecDeque<Chunk>,
}

fn init_lights(mut commands: Commands) {
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

fn create_chunk(
    commands: &mut Commands,
    biome: &mut ResMut<Biome>,
    things: &Res<ObjectCollection>,
) {
    let mut entities = Vec::new();
    let x = match biome.chunks.len() {
        0 => -8,
        _ => match biome.chunks.back() {
            Some(chunk) => chunk.position_x + 1,
            None => -8,
        },
    };

    for z in -10..10 {
        let cube = if x % 2 == 0 {
            match (GAMEPLAY_MIN_Z..=GAMEPLAY_MAX_Z).contains(&z) {
                true => things.light_cube.clone(),
                false => things.light_dimmed_block.clone(),
            }
        } else {
            match (GAMEPLAY_MIN_Z..=GAMEPLAY_MAX_Z).contains(&z) {
                true => things.dark_cube.clone(),
                false => things.dark_dimmed_cube.clone(),
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

    biome.chunks.push_back(Chunk {
        position_x: x,
        entities,
    });
}

fn init_world(
    mut commands: Commands,
    mut biome: ResMut<Biome>,
    mut app_state: ResMut<NextState<AppState>>,
    things: Res<ObjectCollection>,
) {
    for _ in 0..10 {
        create_chunk(&mut commands, &mut biome, &things);
    }

    app_state.set(AppState::Playing);
}

fn check_chunk_visibilities(
    mut commands: Commands,
    entities: Query<&ViewVisibility>,
    mut biome: ResMut<Biome>,
    things: Res<ObjectCollection>,
) {
    // Checks to create new chunks
    if let Some(chunk) = biome.chunks.back() {
        if chunk
            .entities
            .iter()
            .any(|entity| match entities.get(*entity) {
                Ok(visibility) => visibility.get(),
                Err(_) => true,
            })
        {
            create_chunk(&mut commands, &mut biome, &things);
        }
    }

    // Checks to delete old chunks
    if let Some(chunk) = biome.chunks.front() {
        if chunk
            .entities
            .iter()
            .any(|entity| match entities.get(*entity) {
                Ok(visibility) => visibility.get(),
                Err(_) => true,
            })
        {
            return;
        }

        for entity in &chunk.entities {
            if let Some(entity) = commands.get_entity(*entity) {
                entity.despawn_recursive();
            }
        }

        biome.chunks.pop_front();
    }
}
