use bevy::app::App;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens};
use bevy_tweening::{Animator, EaseFunction, Tween};

use crate::constants::{CAMERA_MOVEMENT_SPEED, CAMERA_SPAWN_POINT};
use crate::player::Player;
use crate::states::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, follow_player.run_if(in_state(AppState::Playing)))
            .add_systems(OnEnter(AppState::Clearing), translate_to_spawn);
    }
}

#[derive(Component)]
pub struct Camera;

fn spawn_camera(mut commands: Commands) {
    let mut transform = Transform::from_xyz(-1., 3.5, 4.).looking_at(Vec3::ZERO, Vec3::Y);
    transform.rotate_y(-0.9);
    transform.translation = CAMERA_SPAWN_POINT;

    let animator = Animator::new(Tween::new(
        EaseFunction::CubicInOut,
        CAMERA_MOVEMENT_SPEED * 2,
        TransformScaleLens {
            start: Vec3::new(1.2, 1.2, 1.2),
            end: Vec3::ONE,
        },
    ));

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Orthographic(OrthographicProjection {
                near: -4.,
                scaling_mode: ScalingMode::FixedVertical(2.),
                scale: 2.5,
                ..Default::default()
            }),
            transform,
            tonemapping: Tonemapping::SomewhatBoringDisplayTransform,
            ..Default::default()
        },
        Camera,
        animator,
    ));
}

fn follow_player(
    mut cameras: Query<(&mut Animator<Transform>, &Transform), With<Camera>>,
    players: Query<&Transform, With<Player>>,
) {
    let Some((mut camera_animator, camera_transform)) = cameras.iter_mut().next() else {
        return;
    };

    if camera_transform.scale != Vec3::ONE || camera_animator.tweenable().progress() <= 0.05 {
        return;
    }

    let Some(player_transform) = players.iter().next() else {
        return;
    };

    camera_animator.set_tweenable(Tween::new(
        EaseFunction::QuadraticOut,
        CAMERA_MOVEMENT_SPEED,
        TransformPositionLens {
            start: camera_transform.translation,
            end: Vec3::new(
                player_transform.translation.x + CAMERA_SPAWN_POINT.x,
                CAMERA_SPAWN_POINT.y,
                CAMERA_SPAWN_POINT.z + player_transform.translation.z / 5.,
            ),
        },
    ));
}

fn translate_to_spawn(
    mut cameras: Query<(&mut Animator<Transform>, &mut Transform), With<Camera>>,
) {
    let Some((mut animator, mut transform)) = cameras.iter_mut().next() else {
        return;
    };

    animator.tweenable_mut().set_progress(1.);
    transform.translation = CAMERA_SPAWN_POINT;
}
