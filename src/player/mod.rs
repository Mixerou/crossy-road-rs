use std::collections::VecDeque;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::hierarchy::{BuildChildren, Children, DespawnRecursiveExt};
use bevy::input::ButtonInput;
use bevy::math::{Quat, Vec3};
use bevy::pbr::PbrBundle;
use bevy::prelude::{
    in_state, Commands, Component, Entity, KeyCode, Mesh, OnEnter, Query, Res, ResMut, Transform,
    Visibility, With,
};
use bevy::prelude::{IntoSystemConfigs, SpatialBundle};
use bevy::time::Time;
use bevy_rapier3d::control::{
    CharacterLength, KinematicCharacterController, KinematicCharacterControllerOutput,
};
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::Collider;
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens};
use bevy_tweening::{Animator, AnimatorState, EaseFunction, Sequence, Tracks, Tween};

use crate::constants::{
    FLATTEN_SCALE, GAMEPLAY_MAX_Z, GAMEPLAY_MIN_Z, GLOBAL_GRAVITY, PLAYER_ANIMATION_DURATION,
    PLAYER_CHARACTER_SIZE, PLAYER_JUMP_HEIGHT, PLAYER_MAX_JUMP_QUEUE, PLAYER_MOVE_BACK_KEY_CODES,
    PLAYER_MOVE_FORWARD_KEY_CODES, PLAYER_MOVE_LEFT_KEY_CODES, PLAYER_MOVE_RIGHT_KEY_CODES,
    PLAYER_SPAWN_POINT,
};
use crate::resources::CharacterCollection;
use crate::states::AppState;
use crate::utils;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), spawn)
            .add_systems(OnEnter(AppState::Clearing), despawn)
            .add_systems(
                Update,
                (
                    move_player,
                    handle_move_keys,
                    init_player_move,
                    flatten_player,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
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

#[derive(Default, PartialEq)]
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
    is_initial_jump_made: bool,
}

#[derive(Default, Component)]
pub struct PlayerModel {
    pub rotation_start_at: Option<f32>,
    pub rotation_duration: Option<f32>,
    pub start_rotation: Option<f32>,
    pub end_rotation: Option<f32>,
}

fn spawn(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    characters: Res<CharacterCollection>,
) {
    let child_mesh = meshes
        .get(characters.chicken.mesh.clone())
        .expect("Failed to get character mesh");
    // Let's pretend we didn't know the size
    let child_size = PlayerModelSize::new(utils::calculate_mesh_size(child_mesh));
    let child_scale = PLAYER_CHARACTER_SIZE / child_size.get().max_element();
    let child_translation = Vec3::new(0., -child_size.get().y / 2. * child_scale, 0.);
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
                offset: CharacterLength::Relative(0.001),
                slide: true,
                snap_to_ground: None,
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                PbrBundle {
                    mesh: characters.chicken.mesh.clone(),
                    material: characters.chicken.material.clone(),
                    transform: Transform::from_translation(child_translation)
                        .with_scale(Vec3::splat(child_scale)),
                    visibility: Visibility::Visible,
                    ..Default::default()
                },
                PlayerModel::default(),
                child_size,
                child_animator,
            ));
        });
}

fn despawn(mut commands: Commands, mut players: Query<Entity, With<Player>>) {
    for entity in players.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

fn move_player(
    time: Res<Time>,
    mut players: Query<(&mut Player, &mut KinematicCharacterController, &Children)>,
    mut player_children: Query<(&mut Transform, &mut PlayerModel)>,
    player_controller_outputs: Query<&KinematicCharacterControllerOutput>,
) {
    let Some((mut player, mut controller, children)) = players.iter_mut().next() else {
        return;
    };
    let Some(child) = children.first() else {
        return;
    };
    let Ok((mut child_transform, mut player_model)) = player_children.get_mut(*child) else {
        return;
    };

    if player.velocity.y > -50. {
        player.velocity.y -= GLOBAL_GRAVITY * time.delta_seconds();
    }

    if let Some(controller_output) = player_controller_outputs.iter().next() {
        if player.is_initial_jump_made && !controller_output.collisions.is_empty() {
            player.jump_queue.pop_front();
            player.velocity = Vec3::ZERO;
            player.is_grounded = true;
        }
    }

    player.is_initial_jump_made = true;
    controller.translation = Some(player.velocity * time.delta_seconds());

    if let (
        Some(rotation_start_at),
        Some(rotation_end_at),
        Some(start_rotation),
        Some(end_rotation),
    ) = (
        player_model.rotation_start_at,
        player_model.rotation_duration,
        player_model.start_rotation,
        player_model.end_rotation,
    ) {
        let mut rotation_progress = (time.elapsed_seconds() - rotation_start_at) / rotation_end_at;

        if rotation_progress > 1. {
            rotation_progress = 1.;
            player_model.rotation_start_at = None;
            player_model.rotation_duration = None;
            player_model.start_rotation = None;
            player_model.end_rotation = None;
        }

        let new_rotation =
            start_rotation * (1. - rotation_progress) + end_rotation * rotation_progress;
        child_transform.rotation = Quat::from_rotation_y(new_rotation % TAU);
    }
}

fn handle_move_keys(
    keyboard_input: Res<ButtonInput<KeyCode>>,
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
        let left_queue = player
            .jump_queue
            .iter()
            .filter(|jump| *jump == &PlayerJumpDirection::Left)
            .count() as i8;
        let right_queue = player
            .jump_queue
            .iter()
            .filter(|jump| *jump == &PlayerJumpDirection::Right)
            .count() as i8;

        if player_translation.z.round() as i8 - left_queue + right_queue <= GAMEPLAY_MIN_Z {
            return;
        }

        player.jump_queue.push_back(PlayerJumpDirection::Left);
    } else if keyboard_input.any_just_released(PLAYER_MOVE_RIGHT_KEY_CODES) {
        let left_queue = player
            .jump_queue
            .iter()
            .filter(|jump| *jump == &PlayerJumpDirection::Left)
            .count() as i8;
        let right_queue = player
            .jump_queue
            .iter()
            .filter(|jump| *jump == &PlayerJumpDirection::Right)
            .count() as i8;

        if player_translation.z.round() as i8 - left_queue + right_queue >= GAMEPLAY_MAX_Z {
            return;
        }

        player.jump_queue.push_back(PlayerJumpDirection::Right);
    }
}

fn init_player_move(
    time: Res<Time>,
    mut players: Query<(&mut Player, &Transform, &Children)>,
    mut player_children: Query<(&Transform, &mut PlayerModel)>,
) {
    let Some((mut player, transform, children)) = players.iter_mut().next() else {
        return;
    };

    if !player.is_grounded {
        return;
    }

    let Some(child) = children.first() else {
        return;
    };
    let Ok((child_transform, mut player_model)) = player_children.get_mut(*child) else {
        return;
    };
    let Some(jump_direction) = player.jump_queue.front() else {
        return;
    };

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

    let child_rotation_y = child_transform.rotation.to_scaled_axis().y;

    player_model.rotation_start_at = Some(time.elapsed_seconds());
    player_model.rotation_duration = Some(velocity_y.y / GLOBAL_GRAVITY * 2.);
    player_model.start_rotation = Some(child_rotation_y);
    #[rustfmt::skip]
    let end_rotation = match jump_direction {
        PlayerJumpDirection::Forward => {
            if child_rotation_y.abs() < 0.5
                || (child_rotation_y + PI + FRAC_PI_2).abs() < 0.5
                || (child_rotation_y + FRAC_PI_2).abs() < 0.5
            { Some(-PI) }
            else if (child_rotation_y - TAU).abs() < 0.5
                || (child_rotation_y - FRAC_PI_2).abs() < 0.5
                || (child_rotation_y - PI - FRAC_PI_2).abs() < 0.5
            { Some(PI) }
            else if (child_rotation_y + TAU).abs() < 0.5
            { Some(-TAU - PI) }
            else { None }
        }
        PlayerJumpDirection::Back => {
            if (child_rotation_y.abs() - FRAC_PI_2) < 0.5
                || (child_rotation_y + PI).abs() < 0.5
            { Some(0.) }
            else if (child_rotation_y - PI).abs() < 0.5
                || (child_rotation_y - PI - FRAC_PI_2).abs() < 0.5
            { Some(TAU) }
            else if (child_rotation_y + PI + FRAC_PI_2).abs() < 0.5
            { Some(-TAU) }
            else { None }
        }
        PlayerJumpDirection::Left => {
            if (child_rotation_y - PI).abs() < 0.5
                || (child_rotation_y - TAU).abs() < 0.5
            { Some(PI + FRAC_PI_2) }
            else if (child_rotation_y + PI).abs() < 0.5
                || (child_rotation_y - FRAC_PI_2).abs() < 0.5
                || child_rotation_y.abs() < 0.5
            { Some(-FRAC_PI_2) }
            else if (child_rotation_y + PI + FRAC_PI_2).abs() < 0.5
                || (child_rotation_y + TAU).abs() < 0.5
            { Some(-TAU - FRAC_PI_2) }
            else { None }
        }
        PlayerJumpDirection::Right => {
            if child_rotation_y.abs() < 0.5
                || (child_rotation_y + FRAC_PI_2).abs() < 0.5
                || (child_rotation_y - PI).abs() < 0.5
            { Some(FRAC_PI_2) }
            else if (child_rotation_y + TAU).abs() < 0.5
                || (child_rotation_y + PI).abs() < 0.5
            { Some(-PI - FRAC_PI_2) }
            else if (child_rotation_y - TAU).abs() < 0.5
                || (child_rotation_y - PI - FRAC_PI_2).abs() < 0.5
            { Some(TAU + FRAC_PI_2) }
            else { None }
        }
    };
    player_model.end_rotation = end_rotation;

    player.velocity = velocity_xz + velocity_y;
    player.is_grounded = false;
    player.is_initial_jump_made = false;
}

fn flatten_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
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
    let child_scale = Vec3::splat(PLAYER_CHARACTER_SIZE / child_size.get().max_element());

    if !player.is_grounded || !player.jump_queue.is_empty() {
        child_animator.state = AnimatorState::Paused;
        child_animator.tweenable_mut().set_progress(1.);
        return;
    }

    child_animator.state = AnimatorState::Playing;

    if child_animator.tweenable().progress() < 1. {
        return;
    }

    let (end_position, end_scale) =
        match keyboard_input.any_pressed(utils::get_player_move_key_codes()) {
            true => {
                let end_scale = child_scale * FLATTEN_SCALE;
                let initial_position_y = -child_size.get().y / 2. * child_scale.y;
                let flatten_position_y = -child_size.get().y * end_scale.y / 2.;
                (
                    Vec3::new(
                        0.,
                        initial_position_y + (initial_position_y - flatten_position_y),
                        0.,
                    ),
                    end_scale,
                )
            }
            false => (
                Vec3::new(0., -child_size.get().y / 2. * child_scale.y, 0.),
                child_scale,
            ),
        };

    if child_transform.scale == end_scale && child_transform.translation == end_position {
        return;
    }

    let tracks = Tracks::new([
        Tween::new(
            EaseFunction::CubicOut,
            PLAYER_ANIMATION_DURATION,
            TransformPositionLens {
                start: child_transform.translation,
                end: end_position,
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
