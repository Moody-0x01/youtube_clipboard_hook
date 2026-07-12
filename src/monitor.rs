use std::{thread, time::Duration};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::options as opt;
use notify::{Watcher, RecursiveMode};
use crate::config::load_config;

pub fn monitor_configuration(file: &str, shared_options: Arc<RwLock<opt::Options>>,
    shared_flag: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>>
{
    println!("Watching for file changes...");
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    let config_dir = std::path::Path::new(file).parent().unwrap();
    watcher.watch(config_dir, RecursiveMode::NonRecursive)?;

    for res in rx {
        if !shared_flag.load(Ordering::Relaxed) {
            break ;
        }
        match res {
            Ok(event) => {
                if event.kind.is_modify() {
                    match load_config(file) {
                    Ok(new_config) => {
                        let mut opt = shared_options.write().unwrap();
                        *opt = new_config;
                        println!("Config reloaded successfully!");
                    },
                    Err(e) => {
                        eprintln!("[cphook] failed to parse reloaded config reason: {} {}", e, file);
                    }
                    }
                }
            }
            Err(e) => {
                println!("watch error: {:?}", e);
                return Ok(());
            }
        }
        thread::sleep(Duration::from_millis(1000));
    }
    return Ok(());
}
