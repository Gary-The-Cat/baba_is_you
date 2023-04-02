use std::default;

use bevy::prelude::*;

#[derive(Component)]
pub struct Position{
    pub x: f32,
    pub y: f32,
}

impl Default for Position {
    fn default() -> Self {
        return Position { x: 0.0, y: 0.0 }
    }
}
