use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Color, Commands, Cuboid, Mesh, ResMut, Resource};

use crate::resources::Model;

#[derive(Debug)]
pub struct Ground {
    pub default: Model,
    pub dimmed: Model,
}

#[derive(Debug, Resource)]
pub struct GroundCollection {
    pub light_cube: Ground,
    pub dark_cube: Ground,
}

impl GroundCollection {
    pub(super) fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let cube = meshes.add(Cuboid::from_size(Vec3::splat(1.)));

        let light_green: StandardMaterial =
            Color::rgb(174. / 255., 224. / 255., 102. / 255.).into();
        let dark_green: StandardMaterial = Color::rgb(167. / 255., 217. / 255., 94. / 255.).into();
        let light_dimmed_green = StandardMaterial {
            metallic: 0.5,
            ..light_green.clone()
        };
        let dark_dimmed_green = StandardMaterial {
            metallic: 0.5,
            ..dark_green.clone()
        };

        let light_green = materials.add(light_green);
        let light_dimmed_green = materials.add(light_dimmed_green);
        let dark_green = materials.add(dark_green);
        let dark_dimmed_green = materials.add(dark_dimmed_green);

        let collection = Self {
            light_cube: Ground {
                default: Model::new(cube.clone(), Vec3::ONE, light_green),
                dimmed: Model::new(cube.clone(), Vec3::ONE, light_dimmed_green),
            },
            dark_cube: Ground {
                default: Model::new(cube.clone(), Vec3::ONE, dark_green),
                dimmed: Model::new(cube, Vec3::ONE, dark_dimmed_green),
            },
        };

        commands.insert_resource(collection);
    }
}
