use super::*;

#[derive(Component, Eq, PartialEq, Hash, Clone)]
pub enum Vegetation {
    Corn,
    Bamboo,
    Tree,
    Grass,
}

impl EntityVariant for Vegetation {
    fn spawn(&self, commands: &mut Commands, resources: &Res<Resources>, position: IVec3) {
        commands.spawn((
            SceneBundle {
                scene: resources
                    .entities
                    .get(&Entity::Vegetation(self.clone()))
                    .unwrap()
                    .clone(),
                transform: Transform {
                    translation: Vec3 {
                        x: position.x as f32,
                        y: position.y as f32 - 0.5,
                        z: position.z as f32,
                    },
                    scale: Vec3::splat(1.0),
                    ..Default::default()
                },
                ..default()
            },
            self.clone(),
        ));
    }
}

#[rustfmt::skip]
pub const MODELS: [(Entity, &str); 4] = [
    (Entity::Vegetation(Vegetation::Corn), "models/vegetation_corn.glb#Scene0"),
    (Entity::Vegetation(Vegetation::Bamboo), "models/vegetation_bamboo.glb#Scene0"),
    (Entity::Vegetation(Vegetation::Tree), "models/vegetation_tree.glb#Scene0"),
    (Entity::Vegetation(Vegetation::Grass), "models/vegetation_grass.glb#Scene0"),
];
