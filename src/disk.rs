use serde::Serialize;
use sysinfo::{
    Disks, DiskKind
};

#[derive(Serialize)]
pub struct DiskInfo {
    name: String,
    disk_type: String,
    total_space: String,
    available_space: String,
    usage: String,
    file_system: String
}

/// Gathers relevant info about disks and returns an vector of disks
pub fn disk_info() -> Vec<DiskInfo> {
    let mut disks: Vec<DiskInfo> = vec![];

    for disk in Disks::new_with_refreshed_list().list() {
        // Get's the partition name of the disk
        let disk_name = disk.name().to_str().unwrap().to_string();

        // Gets whether the disk is an SSD, HDD, or Network Drive
        let disk_type_raw = disk.kind();
        let disk_type: String;
        match disk_type_raw {
            DiskKind::HDD => disk_type = String::from("HDD"),
            DiskKind::SSD => disk_type = String::from("SSD"),
            DiskKind::Unknown(_) => disk_type = String::from("Network")
        }

        // Calculates Disk usage;
        let disk_used_space = disk.total_space() - disk.available_space();
    
        disks.push( DiskInfo {
            name: disk_name,
            disk_type: disk_type,
            total_space: disk.total_space().to_disk_units(),
            available_space: disk.available_space().to_disk_units(),
            usage: format!("{:.2}%",((disk_used_space / disk.total_space()) as f64 * 100.0)),
            file_system: disk.file_system().to_str().unwrap_or("Unknown").to_string(),
        });
    }

    disks
}

pub trait ToDiskUnits {
    fn to_disk_units(&self) -> String;
}

impl ToDiskUnits for u64 {
    fn to_disk_units(&self) -> String {
        let len = self.to_string().len();
        let space: f64 = self.clone() as f64;
        match len {
            0..3  => format!("{:.2}BY", space),
            3..6  => format!("{:.2}KB", space / 10.0_f64.powf(3.0)),
            6..9  => format!("{:.2}MB", space / 10.0_f64.powf(6.0)),
            9..12 => format!("{:.2}GB", space / 10.0_f64.powf(9.0)),
            _     => format!("{:.2}TB", space / 10.0_f64.powf(12.0)),
        }
    }
}

impl ToDiskUnits for f64 {
    fn to_disk_units(&self) -> String {
        let len = self.to_string().len();
        match len {
            0..3  => format!("{:.2}BY", self),
            3..6  => format!("{:.2}KB", self / 10.0_f64.powf(3.0)),
            6..9  => format!("{:.2}MB", self / 10.0_f64.powf(6.0)),
            9..12 => format!("{:.2}GB", self / 10.0_f64.powf(9.0)),
            _     => format!("{:.2}TB", self / 10.0_f64.powf(12.0)),
        }
    }
}