use bevy::app::App;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Component)]
pub struct Camera;

fn spawn_camera(mut commands: Commands) {
    let mut transform = Transform::from_xyz(-1.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
    transform.rotate_y(-0.9);

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Orthographic(OrthographicProjection {
                near: -1.,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                scale: 3.,
                ..Default::default()
            }),
            transform,
            tonemapping: Tonemapping::SomewhatBoringDisplayTransform,
            ..Default::default()
        },
        Camera,
    ));
}
