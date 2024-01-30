use std::time::Duration;

use bevy::math::Vec3;
use bevy::prelude::KeyCode;

// Camera
pub const CAMERA_MOVEMENT_SPEED: Duration = Duration::from_secs(1);
pub const CAMERA_SPAWN_POINT: Vec3 = Vec3::new(-1.5, 3.0, 0.75);

// Gameplay
pub const GAMEPLAY_MIN_Z: i8 = -4;
pub const GAMEPLAY_MAX_Z: i8 = 4;

// Objects scaling
pub const FLATTEN_SCALE: Vec3 = Vec3::new(1.125, 0.875, 1.125);

// Player
pub const PLAYER_SPAWN_POINT: Vec3 = Vec3::new(0., 1.01, 0.);
pub const PLAYER_ANIMATION_DURATION: Duration = Duration::from_millis(200);
pub const PLAYER_MAX_JUMP_QUEUE: usize = 2;
pub const PLAYER_JUMP_HEIGHT: f32 = 0.25;
pub const PLAYER_MOVE_FORWARD_KEY_CODES: [KeyCode; 3] = [KeyCode::W, KeyCode::Up, KeyCode::Space];
pub const PLAYER_MOVE_BACK_KEY_CODES: [KeyCode; 2] = [KeyCode::S, KeyCode::Down];
pub const PLAYER_MOVE_LEFT_KEY_CODES: [KeyCode; 2] = [KeyCode::A, KeyCode::Left];
pub const PLAYER_MOVE_RIGHT_KEY_CODES: [KeyCode; 2] = [KeyCode::D, KeyCode::Right];

// Other
pub const GLOBAL_GRAVITY: f32 = 40.;
