mod animation;
mod camera;
mod debug;

mod entities;
mod input;
mod light;
mod player;
mod system_info;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowLevel, WindowMode, WindowPosition, WindowResolution},
};

use animation::AnimationPlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
use entities::EntityPlugin;
use input::InputPlugin;
use light::LightPlugin;
use player::PlayerPlugin;

// ToDo (sorted by priority):
// -> Render only visible blocks
// -> Improve animation logic
// -> Improve the day/night cycle
// -> Fix issues on player rotation (it needs to always looks at the mouse)
// -> Improve debug mode code (enable/disable)
// -> Fix system info module (CPU usage is not accurate, CPU temp is missing, AMD gpus are not supported)
// -> Improve camera (add zoom and movement)

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
        .add_plugins(CameraPlugin)
        .add_plugins(LightPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EntityPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(InputPlugin)
        .run();
}
