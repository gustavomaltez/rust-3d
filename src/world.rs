use bevy::prelude::*;

use crate::asset_loader::SceneAssets;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_world);
    }
}

fn spawn_world(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    for x in -20..=20 {
        for z in -20..=20 {
            let render_bamboo = rand::random::<f32>() > 0.95;
            commands.spawn(SceneBundle {
                scene: scene_assets.grass_block.clone(),
                transform: Transform::from_xyz(x as f32, 0.0, z as f32),
                ..default()
            });
            if render_bamboo {
                commands.spawn(SceneBundle {
                    scene: scene_assets.bamboo.clone(),
                    transform: Transform::from_xyz(x as f32, 1.0, z as f32),
                    ..default()
                });
                let should_render_second_bamboo_on_top = rand::random::<f32>() > 0.5;
                if should_render_second_bamboo_on_top {
                    commands.spawn(SceneBundle {
                        scene: scene_assets.bamboo.clone(),
                        transform: Transform::from_xyz(x as f32, 2.0, z as f32),
                        ..default()
                    });
                }
            }
        }
    }
}
