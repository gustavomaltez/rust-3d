use crate::animation::Animated;

use super::get_animation as core_get_animation;
use super::get_model as core_get_model;
use super::*;

// Core ------------------------------------------------------------------------

pub struct Entity {
    pub coordinates: IVec3,
    pub variant: Variant,
}

#[derive(Component, PartialEq)]
pub enum Variant {
    Player,
}

#[derive(Component, PartialEq)]
pub enum Animation {
    Idle,
    Walk,
}

// Helpers ---------------------------------------------------------------------

pub fn spawn<T: Bundle>(
    commands: &mut Commands,
    resources: &Resources,
    entity: Entity,
    bundle: T,
) {
    commands.spawn((
        SceneBundle {
            scene: get_model(&entity.variant, resources),
            transform: Transform {
                translation: Vec3::new(
                    entity.coordinates.x as f32,
                    entity.coordinates.y as f32 + 0.5,
                    entity.coordinates.z as f32,
                ),
                ..Default::default()
            },
            ..Default::default()
        },
        Animated {
            handle: get_animation(&entity.variant, &Animation::Idle, resources),
        },
        bundle,
    ));
}

// Variants & Animations -------------------------------------------------------

struct ModelData<'a> {
    path: &'a str,
    variant: Variant,
}

struct AnimationData<'a> {
    animation: Animation,
    path: &'a str,
    variant: Variant,
}

const MODELS: [ModelData; 1] = [ModelData {
    path: "models/player.glb#Scene0",
    variant: Variant::Player,
}];

const ANIMATIONS: [AnimationData; 2] = [
    AnimationData {
        animation: Animation::Idle,
        path: "models/player.glb#Animation3",
        variant: Variant::Player,
    },
    AnimationData {
        animation: Animation::Walk,
        path: "models/player.glb#Animation6",
        variant: Variant::Player,
    },
];

pub fn get_model(variant: &Variant, resources: &Resources) -> Handle<Scene> {
    match MODELS.iter().find(|model| model.variant == *variant) {
        Some(model) => core_get_model(model.path.to_string(), resources),
        None => panic!("Error while loading model"),
    }
}

pub fn get_animation(
    variant: &Variant,
    animation: &Animation,
    resources: &Resources,
) -> Handle<AnimationClip> {
    match ANIMATIONS.iter().find(|animation_data| {
        animation_data.variant == *variant
            && animation_data.animation == *animation
    }) {
        Some(animation) => {
            core_get_animation(animation.path.to_string(), resources)
        }
        None => panic!("Error while loading animation"),
    }
}

pub fn load_assets(resources: &mut Resources, asset_server: &Res<AssetServer>) {
    for model in MODELS.iter() {
        load_model(model.path.to_string(), resources, asset_server);
    }
    for animation in ANIMATIONS.iter() {
        load_animation(animation.path.to_string(), resources, asset_server);
    }
}

// Exportable ------------------------------------------------------------------

pub mod exportable {
    pub use super::Animation;
    pub use super::Entity;
    pub use super::Variant;
    pub use super::{get_animation, spawn};
}
