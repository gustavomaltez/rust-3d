use bevy::prelude::*;

// Movement Related Components -------------------------------------------------

#[derive(Component)]
pub struct Velocity {
    value: Vec3,
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3::new(x, y, z),
        }
    }
}

#[derive(Component)]
pub struct Position {
    value: Vec3,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3::new(x, y, z),
        }
    }
}

// Plugin Core -----------------------------------------------------------------

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Position)>, time: Res<Time>) {
    for (velocity, mut position) in query.iter_mut() {
        position.value += velocity.value * time.delta_seconds();
    }
}
