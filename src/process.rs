use serde::Serialize;
use sysinfo::System;

use crate::disk::ToDiskUnits;

#[derive(Serialize)]
pub struct ProccessInfo {
    name: String,
    memory: String,
    run_time: String,
    id: String,
    user_id: String,
    virtual_memory: String
}

impl ProccessInfo {
    pub fn get(system: &System) -> Vec<Self> {
        let mut processes: Vec<Self> = vec![];

        for (process_id, process) in system.processes() {
            processes.push(ProccessInfo {
                name: process.name().to_string_lossy().to_string(),
                memory: process.memory().to_disk_units(),
                run_time: process.run_time().to_runtime(),
                id: process_id.to_string(),
                user_id: process.user_id().unwrap().to_string(),
                virtual_memory: process.virtual_memory().to_disk_units(),
            })
        }

        processes
    }
}

pub trait ToRuntime {
    fn to_runtime(&self) -> String;
}

impl ToRuntime for u64 {
    fn to_runtime(&self) -> String {
        let time = *self as f64;
        match time {
            0.0..60.0    => format!("{:.2}S", time),
            60.0..3600.0 => format!("{:.2}M", time / 60.0),
            _            => format!("{:.2}H", time / 3600.0),
        }
    }
}
