use super::get_model as core_get_model;
use super::*;

// Core ------------------------------------------------------------------------

pub struct Entity {
    pub coordinates: IVec3,
    pub variant: Variant,
}

#[derive(Component, PartialEq)]
pub enum Variant {
    Bamboo,
    Corn,
    Grass,
    Tree,
}

// Helpers ---------------------------------------------------------------------

pub fn spawn(commands: &mut Commands, resources: &Resources, entity: Entity) {
    commands.spawn(SceneBundle {
        scene: get_model(&entity.variant, resources),
        transform: Transform {
            translation: Vec3::new(
                entity.coordinates.x as f32,
                entity.coordinates.y as f32 - 0.5,
                entity.coordinates.z as f32,
            ),
            scale: Vec3::splat(0.5),
            ..Default::default()
        },
        ..Default::default()
    });
}

// Variants & Animations -------------------------------------------------------

struct ModelData<'a> {
    path: &'a str,
    variant: Variant,
}

const MODELS: [ModelData; 4] = [
    ModelData {
        path: "models/vegetation_bamboo.glb#Scene0",
        variant: Variant::Bamboo,
    },
    ModelData {
        path: "models/vegetation_corn.glb#Scene0",
        variant: Variant::Corn,
    },
    ModelData {
        path: "models/vegetation_grass.glb#Scene0",
        variant: Variant::Grass,
    },
    ModelData {
        path: "models/vegetation_tree.glb#Scene0",
        variant: Variant::Tree,
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
