use super::*;

// Core ------------------------------------------------------------------------

#[derive(Component)]
pub struct Vegetation {
    pub coordinates: IVec3,
    pub variant: Variant,
}

impl Vegetation {
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
                translation: Vec3 {
                    x: self.coordinates.x as f32,
                    y: self.coordinates.y as f32 - 0.5,
                    z: self.coordinates.z as f32,
                },
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

// Facade ----------------------------------------------------------------------

const ENTITY_NAME: &str = "vegetation";

struct Facade {}

impl Facade {
    pub fn load_model(
        variant: &Variant,
        resources: &mut Resources,
        asset_server: &Res<AssetServer>,
    ) {
        let model_data = get_variant_model_data(variant);
        let signature = get_model_signature(ENTITY_NAME, &model_data.id);
        load_model(signature, model_data.path, resources, asset_server);
    }

    pub fn get_model(variant: &Variant, resources: &Resources) -> Handle<Scene> {
        let signature = get_model_signature(ENTITY_NAME, &get_variant_model_data(variant).id);
        resources.models[&signature].clone()
    }
}

// Variants & Animations -------------------------------------------------------

struct Data {
    id: String,
    path: String,
}

// Variant -----

#[derive(Component)]
pub enum Variant {
    Bamboo,
    Corn,
    Grass,
    Tree,
}

fn get_variant_model_data(variant: &Variant) -> Data {
    match variant {
        Variant::Grass => Data {
            id: "grass".to_string(),
            path: "models/vegetation_grass.glb#Scene0".to_string(),
        },
        Variant::Bamboo => Data {
            id: "bamboo".to_string(),
            path: "models/vegetation_bamboo.glb#Scene0".to_string(),
        },
        Variant::Tree => Data {
            id: "tree".to_string(),
            path: "models/vegetation_tree.glb#Scene0".to_string(),
        },
        Variant::Corn => Data {
            id: "corn".to_string(),
            path: "models/vegetation_corn.glb#Scene0".to_string(),
        },
    }
}

// External API ----------------------------------------------------------------

pub fn load_assets(resources: &mut Resources, asset_server: &Res<AssetServer>) {
    Facade::load_model(&Variant::Grass, resources, asset_server);
    Facade::load_model(&Variant::Tree, resources, asset_server);
    Facade::load_model(&Variant::Bamboo, resources, asset_server);
    Facade::load_model(&Variant::Corn, resources, asset_server);
}
