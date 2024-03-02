use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    log::tracing_subscriber::fmt::time,
    prelude::*,
};

use sysinfo::{Components, Disks, Networks, System};

use crate::input::InputData;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Update, update)
            .add_systems(Startup, setup)
            .init_resource::<InputData>();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Setup the log text sections
    let font = asset_server.load("fonts/FiraCode-Retina.ttf");
    let font_size = 18.0;
    let style = TextStyle {
        font: font.clone(),
        font_size,
        color: Color::WHITE,
    };
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new("Genesys Ultimate [v0.0.0] (dev)\n", style.clone()),
            TextSection::new("Copyright 2024 GMALTEZ CORP \n", style.clone()),
            TextSection::new("\nCoord: ", style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::new("\nFPS: ", style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::new("\nCPU: ", style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::new("\nGPU: ", style.clone()),
            TextSection::from_style(style.clone()),
        ])
        .with_background_color(Color::rgba(0.0, 0.0, 0.0, 0.5)),
    );
}

fn update(
    diagnostics: Res<DiagnosticsStore>,
    input_data: Res<InputData>,
    mut gizmos: Gizmos,
    mut query: Query<&mut Text>,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average());
    let mut system = System::new_all();
    system.refresh_all();
    let cpu = system.cpus();

    for mut text in query.iter_mut() {
        text.sections[2].value = format!(
            "X: {:.2} Y: {:.2} Z: {:.2}",
            input_data.mouse_position.coordinates.x,
            input_data.mouse_position.coordinates.y,
            input_data.mouse_position.coordinates.z
        );
        text.sections[5].value = format!("{:.0}", fps.unwrap_or(0.0));
        text.sections[8].value = format!(
            "{} {}% {}°C",
            cpu[0].cpu_usage(),
            cpu[0].cpu_usage(),
            cpu[0].cpu_usage()
        );
    }

    gizmos.cuboid(
        Transform {
            translation: Vec3::new(
                input_data.mouse_position.coordinates.x.floor(),
                0.0,
                input_data.mouse_position.coordinates.z.floor(),
            ),
            scale: Vec3::splat(1.0),
            ..Default::default()
        },
        Color::RED,
    );
    gizmos.cuboid(
        Transform {
            translation: Vec3::new(
                input_data.mouse_position.coordinates.x.floor(),
                1.0,
                input_data.mouse_position.coordinates.z.floor(),
            ),
            scale: Vec3::splat(1.0),
            ..Default::default()
        },
        Color::BLUE,
    );
}
