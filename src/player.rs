use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    entity::EntityBundle,
};

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        EntityBundle::new(scene_assets.player.clone(), 0.0, 1.0, 0.0),
        Player,
    ));
}
