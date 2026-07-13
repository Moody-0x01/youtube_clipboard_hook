use std::{thread, time::Duration};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::options as opt;
use crate::downloader::download;
use crate::error_handlers::on_error;
use arboard::{Clipboard};

pub fn spawn_cphookthread(clipboard_options: Arc<RwLock<opt::Options>>,
    clipboard_flag: Arc<AtomicBool>) -> thread::JoinHandle<()>
{
    return thread::spawn(move || {
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
}
