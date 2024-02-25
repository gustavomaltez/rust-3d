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
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform, &mut Animated)>,
) {
    for (_, mut transform, mut animated) in query.iter_mut() {
        let mut translation = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            translation += Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::S) {
            translation -= Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::A) {
            translation -= Vec3::X;
        }
        if keyboard_input.pressed(KeyCode::D) {
            translation += Vec3::X;
        }
        if translation != Vec3::ZERO {
            animated.handle = resources.animations.walk.clone();
        } else {
            animated.handle = resources.animations.idle.clone();
        }
        transform.translation += translation * time.delta_seconds();
    }
}
