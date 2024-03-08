use bevy::prelude::*;

use crate::{
    animation::Animated,
    entities::entity::{character, Resources as EntityResources},
    input::InputData,
};

// Component -------------------------------------------------------------------

#[derive(Component)]
struct Player;

// Plugin ----------------------------------------------------------------------

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, initialize_player)
            .add_systems(Update, player_movement);
    }
}

// Systems ---------------------------------------------------------------------

fn initialize_player(mut commands: Commands, resources: Res<EntityResources>) {
    character::spawn(
        &mut commands,
        &resources,
        character::Entity {
            coordinates: IVec3::new(0, 0, 0),
            variant: character::Variant::Player,
        },
        Player,
    );
}

fn player_movement(
    time: Res<Time>,
    resources: Res<EntityResources>,
    input_data: Res<InputData>,
    mut query: Query<(&mut Transform, &mut Animated), With<Player>>,
) {
    for (mut transform, mut animated) in query.iter_mut() {
        // Rotate the player to look at the mouse position
        let difference =
            input_data.mouse_position.coordinates - transform.translation;
        let angle = f32::atan2(difference.x, difference.z);
        transform.rotation = Quat::from_rotation_y(angle);

        // Move the player based on pressed keys
        let rotation = transform.rotation.clone();

        let mut translation_offset = Vec3::ZERO;
        let mut is_walking = false;

        match input_data.pressed_keys.iter().next() {
            Some(key) => {
                is_walking = true;
                match key {
                    KeyCode::KeyW => {
                        translation_offset += rotation.mul_vec3(Vec3::Z)
                    }
                    KeyCode::KeyS => {
                        translation_offset -= rotation.mul_vec3(Vec3::Z)
                    }
                    KeyCode::KeyA => {
                        translation_offset += rotation.mul_vec3(Vec3::X)
                    }
                    KeyCode::KeyD => {
                        translation_offset -= rotation.mul_vec3(Vec3::X)
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        // Update translation
        let speed_offset = time.delta_seconds() * 5.5;
        transform.translation += translation_offset * speed_offset;

        // Play the walk animation
        animated.handle = if is_walking {
            character::get_animation(
                &character::Variant::Player,
                &character::Animation::Walk,
                &resources,
            )
        } else {
            character::get_animation(
                &character::Variant::Player,
                &character::Animation::Idle,
                &resources,
            )
        };
    }
}
