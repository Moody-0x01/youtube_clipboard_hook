use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicBool;
use clippy_hook::config::load_config;
use clippy_hook::folder_settings::set_download_folder;
use clippy_hook::options as opt;
use clippy_hook::spawner::spawn_cphookthread;
use clippy_hook::monitor::monitor_configuration;
use clippy_hook::logger::GlobalLogger;
// use std::os::unix::net::UnixStream;
fn main() -> Result<(), notify::Error>
{
    // TODO: connect to the stream.
    // TODO: write to it whatever log u might need to write then the server will just write it
    // elseweher.
    // let mut stream = UnixStream::connect("/tmp//clippy_hook.sock")?;
    match GlobalLogger::init_daemon("/tmp/clippy_hook.sock") {
        Ok(_) => GlobalLogger::log("Running in Daemon mode. Connected to FastAPI socket."),
        Err(e) => {
            eprintln!("Failed to connect to socket server: {}. Falling back to CLI mode.", e);
            GlobalLogger::init_cli();
        }
    }
    let file: &str = "/home/moody/.config/cphook/config.json";
    let mut options = load_config(file)
        .expect("[clippy_hook] failed to load the configuration");
    let shared_flag:       Arc<AtomicBool>   = Arc::new(AtomicBool::new(true));
    let clipboard_flag:    Arc<AtomicBool>    = Arc::clone(&shared_flag);
    let shared_options:    Arc<RwLock<opt::Options>>    = Arc::new(RwLock::new(options.clone()));
    let clipboard_options: Arc<RwLock<opt::Options>> = Arc::clone(&shared_options);

    set_download_folder(&mut options.download_path);
    if !options.quiet {
        GlobalLogger::log(&format!("download_path: {}", options.download_path));
    }
    let handle = spawn_cphookthread(clipboard_options, clipboard_flag);
    monitor_configuration(file, shared_options, shared_flag).expect("[clippy_hook] spawn_configuration_monitor failed");
    handle.join().unwrap();
    Ok(())
}
