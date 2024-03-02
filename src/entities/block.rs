use super::*;

#[derive(Component, Eq, PartialEq, Hash, Clone)]
pub enum Block {
    Grass,
    Dirt,
}

impl EntityVariant for Block {
    fn spawn(&self, commands: &mut Commands, resources: &Res<Resources>, position: IVec3) {
        commands.spawn((
            SceneBundle {
                scene: resources
                    .entities
                    .get(&Entity::Block(self.clone()))
                    .unwrap()
                    .clone(),
                transform: Transform {
                    translation: position.as_vec3(),
                    scale: Vec3::splat(0.5),
                    ..Default::default()
                },
                ..default()
            },
            self.clone(),
        ));
    }
}

#[rustfmt::skip]
pub const MODELS: [(Entity, &str); 2] = [
  (Entity::Block(Block::Grass), "models/block_grass.glb#Scene0"),
  (Entity::Block(Block::Dirt),"models/block_dirt.glb#Scene0"),
];
