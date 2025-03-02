use sysinfo::{CpuExt, DiskExt, NetworkExt, NetworksExt, System, SystemExt};
use crate::config::{self, Config};

const DISK_UNIT: DiskUnit = DiskUnit::GB(1.0);

#[derive(serde::Serialize, Debug)]
pub struct HardwareStats {
    pub cpu_usage: u32,
    pub memory_usage: f64,
    pub memory_total: f64,
    pub network_in: f64,
    pub network_out: f64,
    //pub temperature: Option<f32>,
    pub disk_usage: f64,
    pub disk_total: f64,
}

#[derive(PartialEq)]
enum DiskUnit {
    GB(f64),
    MB(f64),
}

fn cpu(sys: &mut System) -> u32 {
    return sys.global_cpu_info().cpu_usage().round() as u32;
}

fn memory(sys: &mut System) -> (f64, f64) {
    let used = sys.used_memory() as f64 / 1_073_741_824.0; // Bytes para GB
    let total = sys.total_memory() as f64 / 1_073_741_824.0; // Bytes para GB
    let memory_usage = (used * 10.0).round() / 10.0;
    let memory_total = (total * 10.0).round() / 10.0;
    (memory_usage, memory_total)
}

fn network(sys: &mut System) -> (f64, f64) {
    let networks = sys.networks();
    let (network_in, network_out) = networks.iter().fold((0, 0), |(acc_in, acc_out), (_, network)| {
        (
            acc_in + network.received(),
            acc_out + network.transmitted(),
        )
    });
    
    // Converte bytes para MB (1 MB = 1_048_576 bytes) e arredonda
    let in_mb = network_in as f64 / 1_048_576.0;  // Bytes para MB
    let out_mb = network_out as f64 / 1_048_576.0; // Bytes para MB

    let network_in = (in_mb * 10.0).round() / 10.0;  // Arredonda para 1 casa decimal
    let network_out = (out_mb * 10.0).round() / 10.0;
    
    (network_in, network_out)
}

fn disk(sys: &mut System, config: &Config) -> (f64, f64) {
    let (disk_used, disk_total) = if config.settings.show_disk {
        let disks = sys.disks();
        disks.get(0).map_or((0.0, 0.0), |disk| {
            let divisor = match DISK_UNIT {
                DiskUnit::GB(_) => 1_073_741_824.0, // 2³⁰ bytes para GB
                DiskUnit::MB(_) => 1_048_576.0,     // 2²⁰ bytes para MB
            };
            let used = (disk.total_space() - disk.available_space()) as f64 / divisor;
            let total = disk.total_space() as f64 / divisor;
            (
                (used * 10.0).round() / 10.0,  // Arredonda para 1 casa decimal
                (total * 10.0).round() / 10.0   // Arredonda para 1 casa decimal
            )
        })
    } else {
        (0.0, 0.0)
    };

    (disk_used, disk_total)
}


pub fn get_hardware_stats(sys: &mut System, config: &Config) -> HardwareStats {
    sys.refresh_system();

    //CPU
    let cpu_usage = cpu(sys);

    //Memory
    let (memory_used, memory_total) = memory(sys);

    //Network
    let (network_in, network_out) = network(sys);

    //Disk
    let (disk_used_gb, disk_total_gb) = disk(sys, config);

    HardwareStats {
    cpu_usage,
    memory_usage: memory_used,
    memory_total: memory_total,
    network_in,
    network_out,
    disk_usage: disk_used_gb,
    disk_total: disk_total_gb,
}
}
