use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

mod block;
mod vegetation;

use block::*;
use vegetation::*;

// Resources -------------------------------------------------------------------

#[derive(Resource, Default)]
pub struct Resources {
    models: HashMap<String, Handle<Scene>>,
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
    vegetation::load_assets(&mut resources, &asset_server);
}

fn spawn_world(mut commands: Commands, resources: Res<Resources>) {
    for x in -15..15 {
        for z in -15..15 {
            if rand::thread_rng().gen_bool(0.3) {
                Block::new(block::Variant::Dirt, IVec3 { x, y: 0, z })
                    .spawn(&mut commands, &resources);
                if rand::thread_rng().gen_bool(0.5) {
                    Vegetation::new(vegetation::Variant::Corn, IVec3 { x, y: 1, z })
                        .spawn(&mut commands, &resources);
                } else if rand::thread_rng().gen_bool(0.1) {
                    Vegetation::new(vegetation::Variant::Tree, IVec3 { x, y: 1, z })
                        .spawn(&mut commands, &resources);
                } else {
                    Vegetation::new(vegetation::Variant::Grass, IVec3 { x, y: 1, z })
                        .spawn(&mut commands, &resources);
                }
            } else {
                Block::new(block::Variant::Grass, IVec3 { x, y: 0, z })
                    .spawn(&mut commands, &resources);
                if rand::thread_rng().gen_bool(0.4) {
                    Vegetation::new(vegetation::Variant::Grass, IVec3 { x, y: 1, z })
                        .spawn(&mut commands, &resources);
                }
            }
        }
    }
}

// Entity Helpers --------------------------------------------------------------

fn load_model(
    signature: String,
    path: String,
    resources: &mut Resources,
    asset_server: &Res<AssetServer>,
) {
    if resources.models.contains_key(&signature) {
        panic!("Model already loaded: {}", signature);
    }
    resources.models.insert(signature, asset_server.load(path));
}

fn get_model_signature(entity: &str, variant: &str) -> String {
    format!("{}_{}", entity, variant)
}
