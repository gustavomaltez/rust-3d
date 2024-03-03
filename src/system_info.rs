use bevy_inspector_egui::egui::Memory;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::enums::device::UsedGpuMemory;
use nvml_wrapper::Device;
use nvml_wrapper::Nvml;
use std::time::Instant;
use sysinfo::{Components, Disks, Networks, System};

pub struct CPUInfo {
    pub name: String,
    pub usage: u32,
}

pub struct GPUInfo {
    pub name: String,
    pub usage: u32,
    pub temperature: u32,
    pub memory: MemoryInfo,
}

pub struct MemoryInfo {
    pub used: u64,
    pub total: u64,
    pub percentage: u64,
}

pub struct SystemInfo {
    pub cpu: CPUInfo,
    pub gpu: GPUInfo,
    pub memory: MemoryInfo,
}

impl SystemInfo {
    pub fn get_system_info() -> SystemInfo {
        SystemInfo {
            cpu: Self::get_cpu_info(),
            gpu: Self::get_gpu_info(),
            memory: Self::get_memory_info(),
        }
    }

    pub fn get_cpu_info() -> CPUInfo {
        let mut sys = sysinfo::System::new_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu();
        let cpu = sys.cpus().get(0).unwrap();
        CPUInfo {
            name: cpu.brand().to_string(),
            usage: cpu.cpu_usage() as u32,
        }
    }

    pub fn get_gpu_info() -> GPUInfo {
        let nvml = Nvml::init().expect("Failed to initialize NVML");
        let device = nvml
            .device_by_index(0)
            .expect("Failed to initialize GPU device");
        let name = device.name().expect("Failed to retrieve GPU brand");
        let utilization = device
            .utilization_rates()
            .expect("Failed to retrieve GPU utilization");
        let memory_info = device
            .memory_info()
            .expect("Failed to retrieve GPU memory info");
        let temperature = device
            .temperature(TemperatureSensor::Gpu)
            .expect("Failed to retrieve GPU temperature");
        GPUInfo {
            name,
            temperature,
            usage: utilization.gpu,
            memory: MemoryInfo {
                used: memory_info.used / 1024 / 1024,
                total: memory_info.total / 1024 / 1024,
                percentage: Self::calculate_memory_percentage(memory_info.used, memory_info.total),
            },
        }
    }

    pub fn get_memory_info() -> MemoryInfo {
        let sys = sysinfo::System::new_all();
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();

        MemoryInfo {
            used: used_memory / 1024 / 1024,
            total: total_memory / 1024 / 1024,
            percentage: Self::calculate_memory_percentage(used_memory, total_memory),
        }
    }

    fn calculate_memory_percentage(used: u64, total: u64) -> u64 {
        ((used as f64 / total as f64) * 100.0) as u64
    }
}
