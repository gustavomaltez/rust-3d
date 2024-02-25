use bevy::prelude::*;

use crate::{animation::Animated, entity::EntityBundle};

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
    mut cursor_moved_events: EventReader<CursorMoved>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(&GlobalTransform, &mut Transform, &mut Animated), With<Player>>,
) {
    let (camera, camera_transform) = camera_query.single();

    for (global_transform, mut transform, mut animated) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        for event in cursor_moved_events.read() {
            let cursor_position = event.position;

            if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                if let Some(distance) = ray.intersect_plane(
                    global_transform.translation(),
                    Plane3d::new(global_transform.up()),
                ) {
                    let point = ray.get_point(distance);
                    direction = point - global_transform.translation();

                    // Use correct method for quaternion creation
                    let rotation = Quat::from_axis_angle(Vec3::Y, direction.x.atan2(direction.z));
                    transform.rotation = rotation;
                }
            }
        }

        println!("Direction: {:?} {:?}", direction.x, direction.z);

        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation += direction * time.delta_seconds();
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation -= direction * time.delta_seconds();
        } else if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation -= direction.cross(Vec3::Y) * time.delta_seconds();
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation += direction.cross(Vec3::Y) * time.delta_seconds();
        }

        if direction != Vec3::ZERO {
            animated.handle = resources.animations.walk.clone();
        } else {
            animated.handle = resources.animations.idle.clone();
        }
    }
}
