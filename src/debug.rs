use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_async_task::{AsyncTaskRunner, AsyncTaskStatus};

use crate::{input::InputData, system_info::SystemInfo};

pub struct DebugPlugin;

#[derive(Resource)]
pub struct SystemInfoData {
    pub info: SystemInfo,
    pub last_refresh: std::time::Instant,
}

impl Default for SystemInfoData {
    fn default() -> Self {
        SystemInfoData {
            info: SystemInfo::get_system_info(),
            last_refresh: std::time::Instant::now(),
        }
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Update, update)
            .add_systems(Update, refresh_system_info)
            .add_systems(Startup, setup)
            .init_resource::<InputData>()
            .init_resource::<SystemInfoData>();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraCode-Retina.ttf");
    let font_size = 18.0;
    let style = TextStyle {
        font: font.clone(),
        font_size,
        color: Color::WHITE,
    };
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Genesys Ultimate [v0.0.0] (dev)\n",
                style.clone(),
            ),
            TextSection::new("Copyright 2024 GMALTEZ CORP \n", style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::from_style(style.clone()),
        ])
        .with_background_color(Color::rgba(0.0, 0.0, 0.0, 0.5)),
    );
}

fn refresh_system_info(
    mut system_info: ResMut<SystemInfoData>,
    mut task_executor: AsyncTaskRunner<SystemInfo>,
) {
    match task_executor.poll() {
        AsyncTaskStatus::Idle => {
            if system_info.last_refresh.elapsed().as_secs() > 5 {
                task_executor.start(async { SystemInfo::get_system_info() });
            }
        }
        AsyncTaskStatus::Pending => {}
        AsyncTaskStatus::Finished(info) => {
            system_info.info = info;
            system_info.last_refresh = std::time::Instant::now();
        }
    }
}

fn update(
    diagnostics: Res<DiagnosticsStore>,
    system_info: ResMut<SystemInfoData>,
    input_data: Res<InputData>,
    mut gizmos: Gizmos,
    mut query: Query<&mut Text>,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average());

    for mut text in query.iter_mut() {
        text.sections[2].value = format!(
            "\nFPS: {:.0} | CPU: {}% ?ºC | GPU: {}% {}ºC",
            fps.unwrap_or(0.0),
            system_info.info.cpu.usage,
            system_info.info.gpu.usage,
            system_info.info.gpu.temperature
        );
        text.sections[3].value =
            format!("\nCPU: {}", system_info.info.cpu.name);
        text.sections[4].value =
            format!("\nGPU: {}", system_info.info.gpu.name);
        text.sections[5].value = format!(
            "\nGPU Memory: {}/{}MB ({:.0}%)",
            system_info.info.gpu.memory.used,
            system_info.info.gpu.memory.total,
            system_info.info.gpu.memory.percentage
        );
        text.sections[6].value = format!(
            "\nMemory: {}/{}MB ({:.0}%)",
            system_info.info.memory.used,
            system_info.info.memory.total,
            system_info.info.memory.percentage
        );
        text.sections[7].value = format!(
            "\n\nBlock X: {:.2} Y: {:.2} Z: {:.2}",
            input_data.mouse_position.coordinates.x,
            input_data.mouse_position.coordinates.y,
            input_data.mouse_position.coordinates.z
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
