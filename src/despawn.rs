use crate::entities::entity::{block, vegetation};

use bevy::{prelude::*, render::view::window};

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn);
    }
}

fn despawn(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    query: Query<
        (Entity, &GlobalTransform),
        Or<(With<block::Entity>, With<vegetation::Entity>)>,
    >,
) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        for (_, camera_transform) in camera_query.iter() {
            let camera_distance =
                camera_transform.translation().distance(Vec3::ZERO);
            if (distance > camera_distance) {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
