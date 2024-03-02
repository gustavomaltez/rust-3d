mod animation;
mod asset_loader;
mod camera;
mod entity;
mod light;
mod movement;
mod player;
mod world;
mod input;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowLevel, WindowMode, WindowPosition, WindowResolution},
};

use animation::AnimationPlugin;
use asset_loader::AssetLoaderPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use light::LightPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;
use input::InputPlugin;

// ToDo:
// -> Retire world.rs and asset_loader.rs (create a logic to spawn blocks)
// -> Add logic to only render the blocks that are visible to the camera
// -> Fix issues on player rotation (it needs to always looks at the mouse)
// -> Try to improve the day/night cycle

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Windowed,
                title: "Genesys Ultimate".into(),
                present_mode: PresentMode::AutoVsync,
                window_level: WindowLevel::AlwaysOnTop,
                position: WindowPosition::At(IVec2::new(1710, 0)),
                resolution: WindowResolution::new(850., 500.),
                ..default()
            }),
            ..default()
        }),))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LightPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(InputPlugin)
        .run();
}
