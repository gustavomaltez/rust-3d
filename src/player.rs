use bevy::prelude::*;

use crate::{animation::Animated, entity::EntityBundle, input::InputData};

// Model -----------------------------------------------------------------------

#[derive(Resource, Default)]
struct Resources {
    model: Handle<Scene>,
    animations: Animations,
}

#[derive(Default)]
struct Animations {
    idle: Handle<AnimationClip>,
    walk: Handle<AnimationClip>,
}

// Component -------------------------------------------------------------------

#[derive(Component)]
struct Player;

// Plugin ----------------------------------------------------------------------

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Resources>()
            .init_resource::<InputData>()
            .add_systems(Startup, load_assets)
            .add_systems(PostStartup, initialize_player)
            .add_systems(Update, player_movement);
    }
}

fn load_assets(mut resources: ResMut<Resources>, asset_server: Res<AssetServer>) {
    *resources = Resources {
        model: asset_server.load("models/Skeleton.glb#Scene0"),
        animations: Animations {
            idle: asset_server.load("models/Skeleton.glb#Animation3"),
            walk: asset_server.load("models/Skeleton.glb#Animation6"),
        },
    };
}

fn initialize_player(mut commands: Commands, resources: Res<Resources>) {
    commands.spawn((
        EntityBundle::new(resources.model.clone(), 0.0, 1.0, 0.0),
        Animated {
            handle: resources.animations.idle.clone(),
        },
        Player,
    ));
}

fn player_movement(
    time: Res<Time>,
    resources: Res<Resources>,
    input_data: Res<InputData>,
    mut query: Query<(&mut Transform, &mut Animated), With<Player>>,
) {
    for (mut transform, mut animated) in query.iter_mut() {
        // Rotate the player to look at the mouse position
        let difference = input_data.mouse_position.coordinates - transform.translation;
        let angle = f32::atan2(difference.x, difference.z);
        transform.rotation = Quat::from_rotation_y(angle);

        // Move the player to the direction the player is looking at
        // W -> Forward | S -> Backward | A -> Left | D -> Right
        let rotation = transform.rotation.clone();
        let speed_offset = time.delta_seconds() * 2.5;
        let mut is_walking = false;

        if input_data.pressed_keys.contains(&KeyCode::KeyW) {
            is_walking = true;
            transform.translation += rotation.mul_vec3(Vec3::Z) * speed_offset
        }
        if input_data.pressed_keys.contains(&KeyCode::KeyS) {
            is_walking = true;
            transform.translation -= rotation.mul_vec3(Vec3::Z) * speed_offset;
        }
        if input_data.pressed_keys.contains(&KeyCode::KeyA) {
            is_walking = true;
            transform.translation += rotation.mul_vec3(Vec3::X) * speed_offset;
        }
        if input_data.pressed_keys.contains(&KeyCode::KeyD) {
            is_walking = true;
            transform.translation -= rotation.mul_vec3(Vec3::X) * speed_offset;
        }

        // Play the walk animation
        if is_walking {
            animated.handle = resources.animations.walk.clone();
        } else {
            animated.handle = resources.animations.idle.clone();
        }
    }
}
