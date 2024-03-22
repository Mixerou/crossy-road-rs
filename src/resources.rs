use bevy::app::{App, Plugin, Update};
use bevy::asset::{AssetServer, Assets, Handle, LoadState, UntypedHandle};
use bevy::math::Vec3;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{
    in_state, run_once, Color, Commands, Cuboid, IntoSystemConfigs, Mesh, NextState, Res, ResMut,
    Resource,
};

use crate::states::AppState;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetLoading>().add_systems(
            Update,
            (setup.run_if(run_once()), check_assets_ready)
                .chain()
                .run_if(in_state(AppState::LoadingModels)),
        );
    }
}

#[derive(Default, Resource)]
pub struct AssetLoading {
    pub handles: Vec<UntypedHandle>,
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    asset_loading: ResMut<AssetLoading>,
) {
    commands.insert_resource(CharacterCollection::new(asset_server, asset_loading));
    commands.insert_resource(ObjectCollection::new(meshes, materials));
}

fn check_assets_ready(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    server: Res<AssetServer>,
    loading: Res<AssetLoading>,
) {
    for handle in &loading.handles {
        match server.load_state(handle.id()) {
            LoadState::Loaded => {}
            LoadState::Failed => panic!("Failed to load some assets"),
            _ => return,
        }
    }

    next_state.set(AppState::InitialisingWorld);
    commands.remove_resource::<AssetLoading>();
}

#[derive(Clone, Debug)]
pub struct Model {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl Model {
    pub fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self { mesh, material }
    }

    pub fn load(
        asset_server: &Res<AssetServer>,
        asset_loading: &mut ResMut<AssetLoading>,
        path: impl ToString,
    ) -> Self {
        let mesh = asset_server.load(format!("models/{}.glb#Mesh0/Primitive0", path.to_string()));
        let material = asset_server.load(format!("models/{}.glb#Material0", path.to_string()));

        asset_loading.handles.push(mesh.clone().untyped());
        asset_loading.handles.push(material.clone().untyped());

        Self { mesh, material }
    }
}

#[derive(Debug, Resource)]
pub struct ObjectCollection {
    pub light_cube: Model,
    pub light_dimmed_block: Model,
    pub dark_cube: Model,
    pub dark_dimmed_cube: Model,
}

impl ObjectCollection {
    pub fn new(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> Self {
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

        Self {
            light_cube: Model::new(cube.clone(), light_green),
            light_dimmed_block: Model::new(cube.clone(), light_dimmed_green),
            dark_cube: Model::new(cube.clone(), dark_green),
            dark_dimmed_cube: Model::new(cube, dark_dimmed_green),
        }
    }
}

#[derive(Clone, Debug, Resource)]
pub struct CharacterCollection {
    pub chicken: Model,
}

impl CharacterCollection {
    pub fn new(asset_server: Res<AssetServer>, mut asset_loading: ResMut<AssetLoading>) -> Self {
        Self {
            chicken: Model::load(&asset_server, &mut asset_loading, "characters/chicken"),
        }
    }
}
