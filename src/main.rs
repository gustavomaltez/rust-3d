mod animation;
mod asset_loader;
mod camera;
mod entity;
mod light;
mod movement;
mod player;
mod world;

use bevy::{
    prelude::*,
    window::{MonitorSelection, PresentMode, WindowMode, WindowPosition, WindowResolution},
};

use animation::AnimationPlugin;
use asset_loader::AssetLoaderPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use light::LightPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

// ToDo:
// -> Retire world.rs and asset_loader.rs (create a logic to spawn blocks)
// -> Create a way to store the mouse position and share it with the player movement system
// -> Create a way to store the keyboard input and share it with the player movement system
// -> Try to improve the day/night cycle

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Genesys Ultimate".into(),
                mode: WindowMode::Windowed,
                present_mode: PresentMode::AutoVsync,
                position: WindowPosition::Centered(MonitorSelection::Index(0)),
                resolution: WindowResolution::new(1400., 700.),
                ..default()
            }),
            ..default()
        }),))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LightPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AnimationPlugin)
        .run();
}