use std::time::Duration;

use bevy::math::Vec3;
use bevy::prelude::KeyCode;

// Camera
pub const CAMERA_MOVEMENT_SPEED: Duration = Duration::from_secs(1);
pub const CAMERA_SPAWN_POINT: Vec3 = Vec3::new(-1.5, 3., 0.75);

// Map
pub const MAP_MIN_X: i32 = -8;
pub const MAP_MIN_Z: i32 = -10;
pub const MAP_MAX_Z: i32 = 10;
pub const MAP_GAMEPLAY_MIN_Z: i32 = -4;
pub const MAP_GAMEPLAY_MAX_Z: i32 = 4;

// Characters scaling
pub const FLATTEN_SCALE: Vec3 = Vec3::new(1.125, 0.875, 1.125);

// Player
pub const PLAYER_SPAWN_POINT: Vec3 = Vec3::new(0., 1.01, 0.);
pub const PLAYER_ANIMATION_DURATION: Duration = Duration::from_millis(200);
pub const PLAYER_MAX_JUMP_QUEUE: usize = 2;
pub const PLAYER_JUMP_HEIGHT: f32 = 0.25;
pub const PLAYER_MOVE_FORWARD_KEY_CODES: [KeyCode; 3] =
    [KeyCode::KeyW, KeyCode::ArrowUp, KeyCode::Space];
pub const PLAYER_MOVE_BACK_KEY_CODES: [KeyCode; 2] = [KeyCode::KeyS, KeyCode::ArrowDown];
pub const PLAYER_MOVE_LEFT_KEY_CODES: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
pub const PLAYER_MOVE_RIGHT_KEY_CODES: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];

// Other
pub const GLOBAL_GRAVITY: f32 = 40.;
