use std::{os::{macos::raw::stat, unix::thread}, sync::mpsc::{channel, Sender}};
use std::thread::sleep;

use config::Config;
use env_logger;
use sysinfo::SystemExt;
mod hardware;
mod config;


fn main() {
    env_logger::init();

    let config = match Config::load("config.toml") {
        Ok(config) => config,
        Err(e) => {
            log::error!("Failed to load config: {}", e);
            return;
        }
    };

    let (tx, rx) = channel();

    spawn_monitor(tx, &config.clone());
}

fn spawn_monitor(tx: Sender<hardware::HardwareStats>, config: &Config) {
    let mut sys = sysinfo::System::new_all();
    loop {
        sys.refresh_all();
        let stats = hardware::get_hardware_stats(&mut sys, config);
        println!("{:?}", stats);

        if let Err(e) = tx.send(stats) {
            log::error!("Failed to send stats: {}", e);
            break;
        }
        sleep(std::time::Duration::from_secs(config.settings.update_interval_secs));
    }
}