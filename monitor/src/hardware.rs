use sysinfo::{CpuExt, DiskExt, NetworksExt, System, SystemExt};
use crate::config::Config;
//use log::debug;


#[derive(Debug, Clone)]
pub struct HardwareStats {
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub memory_total: u64,
    //pub network_usage: f32,
    pub disk_usage: f64,
    pub disk_total: f64,
}


pub fn get_hardware_stats(sys: &mut System, config: &Config) -> HardwareStats {
    sys.refresh_system();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    //TODO: MAKE IT WORK
    //let network_usage = sys.networks_mut().refresh();
    let memory_used_mb = sys.used_memory() / 1024 / 1024;
    let memory_total_mb = sys.total_memory() / 1024 / 1024;

    let (disk_used_gb, disk_total_gb) = if config.settings.show_disk {
        let disks = sys.disks();
        disks.get(0).map_or((0.0, 0.0), |disk| {
            (
                (disk.total_space() - disk.available_space()) as f64 / 1_073_741_824.0, // GB
                disk.total_space() as f64 / 1_073_741_824.0,
            )
        })
    } else {
        (0.0, 0.0)
    };

    HardwareStats {
        cpu_usage,
        memory_usage: memory_used_mb,
        memory_total: memory_total_mb,
        //network_usage: network_usage as f32,
        disk_usage: disk_used_gb,
        disk_total: disk_total_gb,
    }
}
