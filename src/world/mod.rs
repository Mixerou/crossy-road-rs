use bevy::app::{App, Plugin, Startup};
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::pbr::{
    AmbientLight, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle, PbrBundle,
    StandardMaterial,
};
use bevy::prelude::shape::Cube;
use bevy::prelude::{Color, Commands, Mesh, ResMut, Transform};
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::Collider;

use crate::constants::{GAMEPLAY_MAX_Z, GAMEPLAY_MIN_Z};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_world);
    }
}

fn init_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::rgb(219. / 255., 220. / 255., 1.),
        brightness: 1.,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 35_000.,
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

    let cube = meshes.add(Cube::new(1.0).into());

    let light_green: StandardMaterial = Color::rgb(174. / 255., 224. / 255., 102. / 255.).into();
    let dark_green: StandardMaterial = Color::rgb(167. / 255., 217. / 255., 94. / 255.).into();
    let dimmed_light_green = StandardMaterial {
        metallic: 0.5,
        ..light_green.clone()
    };
    let dimmed_dark_green = StandardMaterial {
        metallic: 0.5,
        ..dark_green.clone()
    };

    let light_green = materials.add(light_green);
    let dark_green = materials.add(dark_green);
    let dimmed_light_green = materials.add(dimmed_light_green);
    let dimmed_dark_green = materials.add(dimmed_dark_green);

    for x in -8..50 {
        for z in -10..10 {
            let material = if x % 2 == 0 {
                if (GAMEPLAY_MIN_Z..=GAMEPLAY_MAX_Z).contains(&z) {
                    light_green.clone()
                } else {
                    dimmed_light_green.clone()
                }
            } else {
                if (GAMEPLAY_MIN_Z..=GAMEPLAY_MAX_Z).contains(&z) {
                    dark_green.clone()
                } else {
                    dimmed_dark_green.clone()
                }
            };

            commands.spawn((
                PbrBundle {
                    mesh: cube.clone(),
                    material,
                    transform: Transform::from_xyz(x as f32, 0., z as f32),
                    ..Default::default()
                },
                RigidBody::Fixed,
                Collider::cuboid(0.5, 0.5, 0.5),
            ));
        }
    }
}
