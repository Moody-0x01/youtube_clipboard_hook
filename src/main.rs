// use std::fmt::Error::{Err};
// use std::fmt::Error;
// use std::path::Path;
use std::sync::{Arc, RwLock};
// use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::atomic::AtomicBool;
use crate::config::load_config;
// use std::env::args;

use crate::options as opt;
use crate::spawner::spawn_cphookthread;
use crate::monitor::monitor_configuration;

pub mod options;
pub mod error_handlers;
pub mod downloader;
pub mod folder_settings;
pub mod config;
pub mod spawner;
pub mod consts;
pub mod monitor;

fn main() -> Result<(), notify::Error>
{
    let file: &str = "/home/moody/.config/cphook/config.json";
    let options = load_config(file)
        .expect("[cphook] failed to load the configuration");

    let shared_flag:       Arc<AtomicBool>   = Arc::new(AtomicBool::new(true));
    let clipboard_flag:    Arc<AtomicBool>    = Arc::clone(&shared_flag);
    let shared_options:    Arc<RwLock<opt::Options>>    = Arc::new(RwLock::new(options.clone()));
    let clipboard_options: Arc<RwLock<opt::Options>> = Arc::clone(&shared_options);

    if options.quiet {
        println!("download_path: {}", options.download_path);
    }
    let handle = spawn_cphookthread(clipboard_options, clipboard_flag);
    monitor_configuration(file, shared_options, shared_flag).expect("spawn_configuration_monitor failed");
    handle.join().unwrap();
    Ok(())
}
