use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Bundle)]
pub struct Entity {
    pub velocity: Velocity,
    pub animation: Handle<Animation>,
    pub model: SceneBundle,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
