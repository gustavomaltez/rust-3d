use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

mod block;
mod character;
mod vegetation;

pub mod entity {
    pub use super::block::exportable as block;
    pub use super::character::exportable as character;
    pub use super::vegetation::exportable as vegetation;
    pub use super::Resources;
}

// Resources -------------------------------------------------------------------

#[derive(Resource, Default)]
pub struct Resources {
    models: HashMap<String, Handle<Scene>>,
    animations: HashMap<String, Handle<AnimationClip>>,
}

// Plugin ----------------------------------------------------------------------

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Resources>()
            .add_systems(Startup, load_assets)
            .add_systems(PostStartup, spawn_world);
    }
}

fn load_assets(mut resources: ResMut<Resources>, asset_server: Res<AssetServer>) {
    block::load_assets(&mut resources, &asset_server);
    character::load_assets(&mut resources, &asset_server);
    vegetation::load_assets(&mut resources, &asset_server);
}

fn spawn_world(mut commands: Commands, resources: Res<Resources>) {
    for x in -15..15 {
        for z in -15..15 {
            if rand::thread_rng().gen_bool(0.3) {
                block::spawn(
                    &mut commands,
                    &resources,
                    block::Entity {
                        coordinates: IVec3 { x, y: 0, z },
                        variant: block::Variant::Dirt,
                    },
                );
                if rand::thread_rng().gen_bool(0.5) {
                    vegetation::spawn(
                        &mut commands,
                        &resources,
                        vegetation::Entity {
                            coordinates: IVec3 { x, y: 1, z },
                            variant: vegetation::Variant::Corn,
                        },
                    );
                } else if rand::thread_rng().gen_bool(0.05) {
                    vegetation::spawn(
                        &mut commands,
                        &resources,
                        vegetation::Entity {
                            coordinates: IVec3 { x, y: 1, z },
                            variant: vegetation::Variant::Tree,
                        },
                    );
                } else {
                    vegetation::spawn(
                        &mut commands,
                        &resources,
                        vegetation::Entity {
                            coordinates: IVec3 { x, y: 1, z },
                            variant: vegetation::Variant::Grass,
                        },
                    );
                }
            } else {
                block::spawn(
                    &mut commands,
                    &resources,
                    block::Entity {
                        coordinates: IVec3 { x, y: 0, z },
                        variant: block::Variant::Grass,
                    },
                );
                if rand::thread_rng().gen_bool(0.4) {
                    vegetation::spawn(
                        &mut commands,
                        &resources,
                        vegetation::Entity {
                            coordinates: IVec3 { x, y: 1, z },
                            variant: vegetation::Variant::Grass,
                        },
                    );
                }
            }
        }
    }
}

// Entity Helpers --------------------------------------------------------------

fn get_model(path: String, resources: &Resources) -> Handle<Scene> {
    match resources.models.get(&path) {
        Some(model) => model.clone(),
        None => panic!("Model not loaded: {}", path),
    }
}

fn load_model(path: String, resources: &mut Resources, asset_server: &Res<AssetServer>) {
    if resources.models.contains_key(&path) {
        panic!("Model already loaded: {}", path);
    }
    resources
        .models
        .insert(path.clone(), asset_server.load(path));
}

fn load_animation(path: String, resources: &mut Resources, asset_server: &Res<AssetServer>) {
    if resources.animations.contains_key(&path) {
        panic!("Animation already loaded: {}", path);
    }
    resources
        .animations
        .insert(path.clone(), asset_server.load(path));
}

fn get_animation(path: String, resources: &Resources) -> Handle<AnimationClip> {
    match resources.animations.get(&path) {
        Some(animation) => animation.clone(),
        None => panic!("Animation not loaded: {}", path),
    }
}
