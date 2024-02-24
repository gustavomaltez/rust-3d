mod asset_loader;
mod entity;
mod camera;
mod light;
mod player;
mod world;
mod movement;  

use bevy::{
    prelude::*,
    window::{MonitorSelection, PresentMode, WindowMode, WindowPosition, WindowResolution},
};

use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
use light::LightPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;
use movement::MovementPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Gus game".into(),
                mode: WindowMode::Windowed,
                present_mode: PresentMode::AutoVsync,
                position: WindowPosition::Centered(MonitorSelection::Index(0)),
                resolution: WindowResolution::new(1400., 700.),
                ..default()
            }),
            ..default()
        }),))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LightPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(MovementPlugin)
        .run();
}
