use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ecs::system,
    log::tracing_subscriber::fmt::time,
    prelude::*,
};

use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::enums::device::UsedGpuMemory;
use nvml_wrapper::Device;
use nvml_wrapper::Nvml;
use std::time::Instant;
use sysinfo::{Components, Disks, Networks, System};

use crate::input::InputData;

pub struct DebugPlugin;

// Create resource for system information
#[derive(Resource)]
pub struct SystemInfo {
    pub cpu: String,
    pub gpu_line_a: String,
    pub gpu_line_b: String,
    pub ram: String,
    pub last_refresh: std::time::Instant,
}

impl Default for SystemInfo {
    fn default() -> Self {
        SystemInfo {
            cpu: String::new(),
            gpu_line_a: String::new(),
            gpu_line_b: String::new(),
            ram: String::new(),
            last_refresh: Instant::now(),
        }
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Update, update)
            .add_systems(Startup, setup)
            .init_resource::<InputData>()
            .init_resource::<SystemInfo>();
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
            TextSection::new("\nGPU: ", style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::from_style(style.clone()),
        ])
        .with_background_color(Color::rgba(0.0, 0.0, 0.0, 0.5)),
    );
}

fn update(
    diagnostics: Res<DiagnosticsStore>,
    mut system_info: ResMut<SystemInfo>,
    input_data: Res<InputData>,
    mut gizmos: Gizmos,
    mut query: Query<&mut Text>,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average());

    // Check if the system information needs to be updated (every 2 seconds)
    if system_info.last_refresh.elapsed().as_secs() > 2 {
        let mut sys = System::new_all();
        // Wait a bit because CPU usage is based on diff.
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        // Refresh CPUs again.
        sys.refresh_cpu();
        let cpu = sys.cpus().get(0).unwrap();
        system_info.cpu = format!("{} {:.0}%", cpu.brand(), cpu.cpu_usage());
        // CPU temperature
        let components = Components::new_with_refreshed_list();
        for component in &components {
            println!("{} {}°C", component.label(), component.temperature());
        }
        // Initialize NVML
        let nvml = Nvml::init().expect("Failed to initialize NVML");

        // Get the first GPU device

        let device = nvml
            .device_by_index(0)
            .expect("Failed to initialize GPU device");

        // Get GPU brand
        let brand = device.name().expect("Failed to retrieve GPU brand");

        let utilization = device
            .utilization_rates()
            .expect("Failed to retrieve GPU utilization");

        // Get GPU memory info
        let memory_info = device
            .memory_info()
            .expect("Failed to retrieve GPU memory info");

        // Get GPU temperature
        let temperature = device
            .temperature(TemperatureSensor::Gpu)
            .expect("Failed to retrieve GPU temperature");

        system_info.gpu_line_a = format!("{} {:.0}% {:.0}°C", brand, utilization.gpu, temperature);
        system_info.gpu_line_b = format!(
            "{}/{}MB ({:.0}%)",
            memory_info.used / 1024 / 1024,
            memory_info.total / 1024 / 1024,
            memory_info.used as f32 / memory_info.total as f32 * 100.0,
        );
        system_info.last_refresh = std::time::Instant::now();
    }

    for mut text in query.iter_mut() {
        text.sections[2].value = format!(
            "X: {:.2} Y: {:.2} Z: {:.2}",
            input_data.mouse_position.coordinates.x,
            input_data.mouse_position.coordinates.y,
            input_data.mouse_position.coordinates.z
        );
        text.sections[5].value = format!("{:.0}", fps.unwrap_or(0.0));
        text.sections[7].value = system_info.cpu.clone();
        text.sections[9].value = system_info.gpu_line_a.clone();
        text.sections[11].value = system_info.gpu_line_b.clone();
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
