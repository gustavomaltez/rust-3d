use bevy::prelude::*;

use crate::{movement::Position, movement::Velocity};

#[derive(Bundle)]
pub struct EntityBundle {
    velocity: Velocity,
    position: Position,
    model: SceneBundle,
}

impl EntityBundle {
    pub fn new(scene: Handle<Scene>, x: f32, y: f32, z: f32) -> Self {
        Self {
            velocity: Velocity::new(0.0, 0.0, 0.0),
            position: Position::new(x, y, z),
            model: SceneBundle {
                scene,
                transform: Transform::from_xyz(x, y, z),
                ..Default::default()
            },
        }
    }
}
