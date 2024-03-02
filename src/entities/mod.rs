use bevy::prelude::*;
use std::collections::HashMap;

mod block;
mod vegetation;

use block::*;
use vegetation::*;

// Components ------------------------------------------------------------------

#[derive(Component, Eq, PartialEq, Hash, Clone)]
enum Entity {
    Block(Block),
    Vegetation(Vegetation),
}

trait EntityVariant {
    fn spawn(&self, commands: &mut Commands, resources: &Res<Resources>, position: IVec3);
}

// Plugin ----------------------------------------------------------------------

#[derive(Resource, Default)]
pub struct Resources {
    entities: HashMap<Entity, Handle<Scene>>,
}

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Resources>()
            .add_systems(Startup, load_assets)
            .add_systems(PostStartup, initialize_world);
    }
}

fn load_assets(mut resources: ResMut<Resources>, asset_server: Res<AssetServer>) {
    let models_to_load = block::MODELS.iter().chain(vegetation::MODELS.iter());
    for (entity, path) in models_to_load {
        resources
            .entities
            .insert(entity.clone(), asset_server.load(path.to_string()));
    }
}

fn spawn_entity<T: EntityVariant>(
    commands: &mut Commands,
    resources: &Res<Resources>,
    entity: T,
    position: IVec3,
) {
    entity.spawn(commands, resources, position);
}

fn initialize_world(mut commands: Commands, resources: Res<Resources>) {
    for x in -10..10 {
        for z in -10..10 {
            spawn_entity(&mut commands, &resources, Block::Grass, IVec3::new(x, 0, z));
            let spawn_corn = rand::random::<f32>() < 0.1;
            if spawn_corn {
                spawn_entity(
                    &mut commands,
                    &resources,
                    Vegetation::Corn,
                    IVec3::new(x, 1, z),
                );
            } else {
                // spawn_entity(
                //     &mut commands,
                //     &resources,
                //     Vegetation::Grass,
                //     IVec3::new(x, 1, z),
                // );
            }
        }
    }

    spawn_entity(
        &mut commands,
        &resources,
        Vegetation::Tree,
        IVec3::new(5, 1, -2),
    );
}
