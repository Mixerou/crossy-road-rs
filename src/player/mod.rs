use std::collections::VecDeque;

use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::Assets;
use bevy::hierarchy::{BuildChildren, Children};
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::shape::Cube;
use bevy::prelude::{
    Color, Commands, Component, KeyCode, Mesh, Query, Res, ResMut, Transform, Visibility,
};
use bevy::prelude::{IntoSystemConfigs, SpatialBundle};
use bevy::time::Time;
use bevy_rapier3d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::Collider;
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens};
use bevy_tweening::{Animator, AnimatorState, EaseFunction, Sequence, Tracks, Tween};

use crate::constants::{
    FLATTEN_SCALE, GAMEPLAY_MAX_Z, GAMEPLAY_MIN_Z, GLOBAL_GRAVITY, PLAYER_ANIMATION_DURATION,
    PLAYER_JUMP_HEIGHT, PLAYER_MAX_JUMP_QUEUE, PLAYER_MOVE_BACK_KEY_CODES,
    PLAYER_MOVE_FORWARD_KEY_CODES, PLAYER_MOVE_LEFT_KEY_CODES, PLAYER_MOVE_RIGHT_KEY_CODES,
    PLAYER_SPAWN_POINT,
};
use crate::utils;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn).add_systems(
            Update,
            ((move_player, handle_move_keys, jump_player, flatten_player).chain(),),
        );
    }
}

#[derive(Component)]
pub struct PlayerModelSize(Vec3);

impl PlayerModelSize {
    pub fn new(size: Vec3) -> Self {
        Self(size)
    }

    pub fn get(&self) -> Vec3 {
        self.0
    }
}

#[derive(Default)]
pub enum PlayerJumpDirection {
    #[default]
    Forward,
    Back,
    Left,
    Right,
}

#[derive(Default, Component)]
pub struct Player {
    jump_queue: VecDeque<PlayerJumpDirection>,
    velocity: Vec3,
    is_grounded: bool,
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let child_mesh = Cube::new(0.5).into();
    // Let's pretend we didn't know the size
    let child_size = PlayerModelSize::new(utils::calculate_mesh_size(&child_mesh));
    let child_translation = Vec3::new(0., (child_size.get().y - 1.) / 2., 0.);
    let child_animator = Animator::new(Tween::new(
        EaseFunction::CubicInOut,
        PLAYER_ANIMATION_DURATION,
        TransformPositionLens {
            start: child_translation,
            end: child_translation,
        },
    ));

    commands
        .spawn((
            Player::default(),
            SpatialBundle::from_transform(Transform::from_translation(PLAYER_SPAWN_POINT)),
            RigidBody::KinematicPositionBased,
            Collider::cuboid(0.5, 0.5, 0.5),
            KinematicCharacterController {
                slide: true,
                snap_to_ground: None,
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                PbrBundle {
                    mesh: meshes.add(child_mesh),
                    material: materials
                        .add(Color::rgb(100. / 255., 200. / 255., 50. / 255.).into()),
                    transform: Transform::from_translation(child_translation),
                    visibility: Visibility::Visible,
                    ..Default::default()
                },
                child_size,
                child_animator,
            ));
        });
}

fn move_player(
    time: Res<Time>,
    mut players: Query<(&mut Player, &mut KinematicCharacterController)>,
    player_controller_outputs: Query<&KinematicCharacterControllerOutput>,
) {
    let Some((mut player, mut controller)) = players.iter_mut().next() else {
        return;
    };

    if player.velocity.y > -50. {
        player.velocity.y -= GLOBAL_GRAVITY * time.delta_seconds();
    }

    if let Some(controller_output) = player_controller_outputs.iter().next() {
        if !controller_output.collisions.is_empty() {
            player.jump_queue.pop_front();
            player.velocity = Vec3::ZERO;
            player.is_grounded = true;
        }
    }

    controller.translation = Some(player.velocity * time.delta_seconds());
}

fn handle_move_keys(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Player, &Transform)>,
) {
    let Some((mut player, player_transform)) = players.iter_mut().next() else {
        return;
    };
    let player_translation = player_transform.translation;

    if player.jump_queue.len() >= PLAYER_MAX_JUMP_QUEUE {
        return;
    }

    if keyboard_input.any_just_released(PLAYER_MOVE_FORWARD_KEY_CODES) {
        player.jump_queue.push_back(PlayerJumpDirection::Forward);
    } else if keyboard_input.any_just_released(PLAYER_MOVE_BACK_KEY_CODES) {
        player.jump_queue.push_back(PlayerJumpDirection::Back);
    } else if keyboard_input.any_just_released(PLAYER_MOVE_LEFT_KEY_CODES) {
        if player_translation.z.round() as i8 - player.jump_queue.len() as i8 <= GAMEPLAY_MIN_Z {
            return;
        }
        player.jump_queue.push_back(PlayerJumpDirection::Left);
    } else if keyboard_input.any_just_released(PLAYER_MOVE_RIGHT_KEY_CODES) {
        if player_translation.z.round() as i8 + player.jump_queue.len() as i8 >= GAMEPLAY_MAX_Z {
            return;
        }
        player.jump_queue.push_back(PlayerJumpDirection::Right);
    }
}

fn jump_player(
    time: Res<Time>,
    mut players: Query<(&mut Player, &mut KinematicCharacterController, &Transform)>,
) {
    let Some((mut player, mut controller, transform)) = players.iter_mut().next() else {
        return;
    };
    let Some(jump_direction) = player.jump_queue.front() else {
        return;
    };

    if !player.is_grounded {
        return;
    }

    let player_translation = transform.translation;
    let target_x = match jump_direction {
        PlayerJumpDirection::Forward => (player_translation.x + 1.).round() - player_translation.x,
        PlayerJumpDirection::Back => (player_translation.x - 1.).round() - player_translation.x,
        _ => 0.,
    };
    let target_z = match jump_direction {
        PlayerJumpDirection::Left => (player_translation.z - 1.).round() - player_translation.z,
        PlayerJumpDirection::Right => (player_translation.z + 1.).round() - player_translation.z,
        _ => 0.,
    };

    let displacement_y = 0.;
    let displacement_xz = Vec3::new(target_x, 0., target_z);

    let velocity_y = Vec3::Y * f32::sqrt(-2. * -GLOBAL_GRAVITY * PLAYER_JUMP_HEIGHT);
    let velocity_xz = displacement_xz
        / (f32::sqrt(-2. * PLAYER_JUMP_HEIGHT / -GLOBAL_GRAVITY)
            + f32::sqrt(2. * (displacement_y - PLAYER_JUMP_HEIGHT) / -GLOBAL_GRAVITY));

    player.velocity = velocity_xz + velocity_y;
    player.is_grounded = false;
    controller.translation = Some(player.velocity * time.delta_seconds());
}

fn flatten_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &Children)>,
    mut player_children: Query<(&Transform, &PlayerModelSize, &mut Animator<Transform>)>,
) {
    let Some((player, children)) = players.iter_mut().next() else {
        return;
    };
    let Some(child) = children.first() else {
        return;
    };
    let Ok((child_transform, child_size, mut child_animator)) = player_children.get_mut(*child)
    else {
        return;
    };

    if !player.is_grounded || !player.jump_queue.is_empty() {
        child_animator.state = AnimatorState::Paused;
        child_animator.tweenable_mut().set_progress(1.);
        return;
    }

    child_animator.state = AnimatorState::Playing;

    if child_animator.tweenable().progress() < 1. {
        return;
    }

    let (end_position_y, end_scale) =
        match keyboard_input.any_pressed(utils::get_player_move_key_codes()) {
            true => (
                (child_size.get().y * FLATTEN_SCALE.y - 1.) / 2.,
                FLATTEN_SCALE,
            ),
            false => ((child_size.get().y - 1.) / 2., Vec3::ONE),
        };

    if child_transform.translation.y == end_position_y && child_transform.scale == end_scale {
        return;
    }

    let tracks = Tracks::new([
        Tween::new(
            EaseFunction::CubicOut,
            PLAYER_ANIMATION_DURATION,
            TransformPositionLens {
                start: child_transform.translation,
                end: Vec3::new(
                    child_transform.translation.x,
                    end_position_y,
                    child_transform.translation.z,
                ),
            },
        ),
        Tween::new(
            EaseFunction::CubicOut,
            PLAYER_ANIMATION_DURATION,
            TransformScaleLens {
                start: child_transform.scale,
                end: end_scale,
            },
        ),
    ]);
    let sequence = Sequence::new([tracks]);

    child_animator.set_tweenable(sequence);
}
