use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub player: Handle<Scene>,
    pub grass_block: Handle<Scene>,
    pub bamboo: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        player: asset_server.load("models/Skeleton.glb#Scene0"),
        grass_block: asset_server.load("models/Grass Block.glb#Scene0"),
        bamboo: asset_server.load("models/Bamboo.glb#Scene0"),
    };
}
