use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicBool;
use clippy_hook::config::load_config;
use clippy_hook::options as opt;
use clippy_hook::spawner::spawn_cphookthread;
use clippy_hook::monitor::monitor_configuration;

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
