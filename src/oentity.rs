use bevy::prelude::*;

use crate::{movement::Position, movement::Velocity};

#[derive(Bundle)]
pub struct EntityBundle {
    velocity: Velocity,
    position: Position,
    model: SceneBundle,
}

impl EntityBundle {
    pub fn new(scene: Handle<Scene>, coords: IVec3) -> Self {
        Self {
            velocity: Velocity::new(0.0, 0.0, 0.0),
            position: Position::new(coords.as_vec3()),
            model: SceneBundle {
                scene,
                transform: Transform {
                    translation: Vec3::new(coords.x as f32, coords.y as f32 - 0.5, coords.z as f32),
                    scale: Vec3::splat(1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
