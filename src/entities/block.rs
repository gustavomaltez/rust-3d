use super::get_model as core_get_model;
use super::*;

// Core ------------------------------------------------------------------------

#[derive(Component)]
pub struct Entity {
    pub coordinates: IVec3,
    pub variant: Variant,
}

#[derive(Component, PartialEq)]
pub enum Variant {
    Grass,
    Dirt,
}

// Helpers ---------------------------------------------------------------------

pub fn spawn(commands: &mut Commands, resources: &Resources, entity: Entity) {
    commands.spawn((
        SceneBundle {
            scene: get_model(&entity.variant, resources),
            transform: Transform {
                translation: entity.coordinates.as_vec3(),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            ..Default::default()
        },
        entity,
    ));
}

// Variants & Animations -------------------------------------------------------

struct ModelData<'a> {
    path: &'a str,
    variant: Variant,
}

const MODELS: [ModelData; 2] = [
    ModelData {
        path: "models/block_grass.glb#Scene0",
        variant: Variant::Grass,
    },
    ModelData {
        path: "models/block_dirt.glb#Scene0",
        variant: Variant::Dirt,
    },
];

pub fn get_model(variant: &Variant, resources: &Resources) -> Handle<Scene> {
    match MODELS.iter().find(|model| model.variant == *variant) {
        Some(model) => core_get_model(model.path.to_string(), resources),
        None => panic!("Error while loading model"),
    }
}

pub fn load_assets(resources: &mut Resources, asset_server: &Res<AssetServer>) {
    for model in MODELS.iter() {
        load_model(model.path.to_string(), resources, asset_server);
    }
}

// Exportable ------------------------------------------------------------------

pub mod exportable {
    pub use super::spawn;
    pub use super::Entity;
    pub use super::Variant;
}
