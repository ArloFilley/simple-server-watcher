use serde::Serialize;
use sysinfo::System;

use crate::{
    disk::{disk_info, DiskInfo, ToDiskUnits},
    process::{ProccessInfo, ToRuntime},
};

#[derive(Serialize)]
pub struct SystemInfo {
    key: String,
    host_name: String,
    uptime: String,
    os: String,
    total_ram: String,
    used_ram: String,
    available_ram: String,
    ram_usage: String,
    total_swap: String,
    used_swap: String,
    available_swap: String,
    swap_usage: String,
    disks: Vec<DiskInfo>,
    processes: Vec<ProccessInfo>,
}

impl SystemInfo {
    pub fn get(system: &mut System, key: String) -> Self {
        SystemInfo {
            key: key,
            host_name: System::host_name().unwrap_or(String::from("Unkown")),
            uptime: System::uptime().to_runtime(),
            os: System::long_os_version().unwrap_or(String::from("Unkown")),
            total_ram: system.total_memory().to_disk_units(),
            used_ram: system.used_memory().to_disk_units(),
            available_ram: system.available_memory().to_disk_units(),
            ram_usage: (system.used_memory() / system.total_memory() * 100).to_string(),
            total_swap: system.total_swap().to_disk_units(),
            used_swap: system.used_swap().to_disk_units(),
            available_swap: system.free_swap().to_disk_units(),
            swap_usage: (system.used_swap() / system.total_swap() * 100).to_string(),
            disks: disk_info(),
            processes: ProccessInfo::get(&system),
        }
    }
}
