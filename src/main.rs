// use std::fmt::Error::{Err};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::error::Error;
// use std::fmt::Error;
// use std::path::Path;
use std::{thread, time::Duration};
use std::fs::File;
use std::io::BufReader;

use notify::{Watcher, RecursiveMode};
use arboard::{Clipboard};

use crate::downloader::download;
use crate::error_handlers::on_error;
use crate::options as opt;
// use std::env::args;

pub mod options;
pub mod error_handlers;
pub mod downloader;
pub mod folder_settings;
// pub mod config;



#[allow(dead_code)]
fn load_config(path: &str) -> Result<opt::Options, Box<dyn Error>> {
    // The `?` operator automatically propagates the error up if something fails
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    
    Ok(config)
}

fn main() -> Result<(), notify::Error>
{
    let file: &str = "/home/moody/.config/cphook/config.json";
    // 1. Properly unwrap the watcher right here so it stays alive!
    // 1. Load initial options and wrap them in an Arc + RwLock
    let options = load_config(file)
        .expect("[cphook] failed to load the configuration");
    // let flag = false;

    let shared_flag    = Arc::new(AtomicBool::new(true));
    let shared_options = Arc::new(RwLock::new(options.clone()));
    let clipboard_options = Arc::clone(&shared_options);
    let clipboard_flag    = Arc::clone(&shared_flag);
    println!("cfg: {:?}", shared_options);

    if options.quiet {
        println!("download_path: {}", options.download_path);
    }

    let handle = thread::spawn(move || {
        let mut links: Vec<String> = Vec::new();
        loop {
            let opt = clipboard_options.read().unwrap();
            {
                match Clipboard::new() {
                    Ok(mut clip) => {
                        match clip.get_text() {
                            Ok(new) => download(&new, &mut links, &opt),
                            Err(e) => on_error(e, "get_text")
                        }
                    },
                    Err(e) => {
                        clipboard_flag.store(true, Ordering::Relaxed);
                        println!("[cphook] clipboard daemon failed reason: {}", e);
                        break ;
                    }
                }
            } // The read-lock drops automatically here at the end of this block
            thread::sleep(Duration::from_millis(1000));
        }
    });
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
            Err(e) => println!("watch error: {:?}", e),
        }
        thread::sleep(Duration::from_millis(1000));
    }
    handle.join().unwrap();
    Ok(())
}
