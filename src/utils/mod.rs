use bevy::math::Vec3;
use bevy::prelude::{KeyCode, Mesh};
use bevy::render::mesh::VertexAttributeValues;

use crate::constants::{
    PLAYER_MOVE_BACK_KEY_CODES, PLAYER_MOVE_FORWARD_KEY_CODES, PLAYER_MOVE_LEFT_KEY_CODES,
    PLAYER_MOVE_RIGHT_KEY_CODES,
};

pub fn calculate_mesh_size(mesh: &Mesh) -> Vec3 {
    let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    else {
        return Vec3::ZERO;
    };

    let mut min = Vec3::splat(f32::INFINITY);
    let mut max = Vec3::splat(f32::NEG_INFINITY);

    for position in positions {
        min = min.min(Vec3::new(position[0], position[1], position[2]));
        max = max.max(Vec3::new(position[0], position[1], position[2]));
    }

    max - min
}

pub fn get_player_move_key_codes() -> Vec<KeyCode> {
    [
        PLAYER_MOVE_FORWARD_KEY_CODES.as_slice(),
        PLAYER_MOVE_BACK_KEY_CODES.as_slice(),
        PLAYER_MOVE_LEFT_KEY_CODES.as_slice(),
        PLAYER_MOVE_RIGHT_KEY_CODES.as_slice(),
    ]
    .concat()
}
