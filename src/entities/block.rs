use super::*;

// Core ------------------------------------------------------------------------

#[derive(Component)]
pub struct Block {
    pub coordinates: IVec3,
    pub variant: Variant,
}

impl Block {
    pub fn new(variant: Variant, coordinates: IVec3) -> Self {
        Self {
            variant,
            coordinates,
        }
    }

    pub fn spawn(&self, commands: &mut Commands, resources: &Resources) {
        commands.spawn(SceneBundle {
            scene: Facade::get_model(&self.variant, &resources),
            transform: Transform {
                translation: self.coordinates.as_vec3(),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

// Facade ----------------------------------------------------------------------

const ENTITY_NAME: &str = "block";

struct Facade {}

impl Facade {
    pub fn load_model(
        variant: &Variant,
        resources: &mut Resources,
        asset_server: &Res<AssetServer>,
    ) {
        let model_data = get_variant_model(variant);
        let signature = get_model_signature(ENTITY_NAME, &model_data.id);
        load_model(signature, model_data.path, resources, asset_server);
    }

    pub fn load_animation(
        variant: &Variant,
        animation: &Animation,
        resources: &mut Resources,
        asset_server: &Res<AssetServer>,
    ) {
        let model_data = get_variant_animation(variant, animation);
        let model_signature = get_model_signature(ENTITY_NAME, &model_data.id);
        let signature = get_animation_signature(model_signature.as_str(), &model_data.id);
        load_animation(signature, model_data.path, resources, asset_server);
    }

    pub fn get_model(variant: &Variant, resources: &Resources) -> Handle<Scene> {
        let signature = get_model_signature(ENTITY_NAME, &get_variant_model(variant).id);
        resources.models[&signature].clone()
    }
}

// Variants & Animations -------------------------------------------------------

struct BlockData {
    id: String,
    path: String,
}

// Variant -----

#[derive(Component)]
pub enum Variant {
    Grass,
    Dirt,
}

fn get_variant_model(variant: &Variant) -> BlockData {
    match variant {
        Variant::Grass => BlockData {
            id: "grass".to_string(),
            path: "models/block_grass.glb#Scene0".to_string(),
        },
        Variant::Dirt => BlockData {
            id: "dirt".to_string(),
            path: "models/block_dirt.glb#Scene0".to_string(),
        },
    }
}

// Animation -----

pub enum Animation {
    Idle,
}

fn get_variant_animation(variant: &Variant, animation: &Animation) -> BlockData {
    match (variant, animation) {
        (Variant::Grass, Animation::Idle) => BlockData {
            id: "idle".to_string(),
            path: "models/block_grass.glb#Animation0".to_string(),
        },
        (Variant::Dirt, Animation::Idle) => BlockData {
            id: "idle".to_string(),
            path: "models/block_dirt.glb#Animation0".to_string(),
        },
    }
}

// External API ----------------------------------------------------------------

pub fn load_assets(resources: &mut Resources, asset_server: &Res<AssetServer>) {
    // Grass
    Facade::load_model(&Variant::Grass, resources, asset_server);
    // Facade::load_animation(&Variant::Grass, &Animation::Idle, resources, asset_server);

    // Dirt
    Facade::load_model(&Variant::Dirt, resources, asset_server);
}
