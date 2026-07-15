use std::{thread, time::Duration};
use std::time::{Instant};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::options as opt;
use notify::{Watcher, RecursiveMode};
use crate::config::load_config;
use crate::logger::GlobalLogger;
use crate::folder_settings::set_download_folder;

pub fn monitor_configuration(file: &str, shared_options: Arc<RwLock<opt::Options>>,
    shared_flag: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>>
{
    println!("[clippy_hook] config Monitor started for {}", file);
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    let config_dir = std::path::Path::new(file).parent().unwrap();
    let mut last_reload = Instant::now() - Duration::from_secs(1);
    let cooldown = Duration::from_millis(200);

    watcher.watch(config_dir, RecursiveMode::NonRecursive)?;
    for res in rx {
        if !shared_flag.load(Ordering::Relaxed) {
            break ;
        }
        if last_reload.elapsed() < cooldown {
            continue;
        }
        match res {
            Ok(event) => {
                if event.kind.is_modify() {
                    thread::sleep(Duration::from_millis(200));
                    match load_config(file) {
                    Ok(mut new_config) => {
                        match shared_options.try_write() {
                            Ok(mut opt) => {
                                set_download_folder(&mut new_config.download_path);
                                if new_config.active != opt.active {
                                    if new_config.active {
                                        println!("[clippy_hook] Activating clipboard monitoring");
                                        GlobalLogger::log("[clippy_hook] Activating clipboard monitoring");
                                    } else {
                                        println!("[clippy_hook] Deactivating clipboard monitoring");
                                        GlobalLogger::log("[clippy_hook] Deactivating clipboard monitoring");
                                    }
                                } else {
                                    println!("[clippy_hook] Config reloaded successfully!");
                                    GlobalLogger::log("[clippy_hook] Config reloaded successfully!");
                                }
                                *opt = new_config;
                                last_reload = Instant::now();
                            }
                            Err(_) => {
                                eprintln!("[clippy_hook] Warning: Options are locked by another thread! Skipping reload.");
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("[clippy_hook] failed to parse reloaded config reason: {} {}", e, file);
                    }
                    }
                }
            }
            Err(e) => {
                println!("[clippy_hook] watch error: {:?}", e);
                shared_flag.store(false, Ordering::Relaxed);
                return Ok(());
            }
        }
    }
    println!("[clippy_hook] config Monitor stopped for {}", file);
    return Ok(());
}
