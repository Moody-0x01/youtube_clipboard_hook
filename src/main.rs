use std::{thread, time::Duration};
use arboard::{Clipboard};
use crate::folder_settings::set_download_folder;
use crate::downloader::download_video;
use crate::error_handlers::on_error;
use std::env::args;

pub mod error_handlers;
pub mod downloader;
pub mod folder_settings;

fn main() {
    let options: Vec<String> = args().collect();
    let mut links: Vec<String> = Vec::new();

    if options.len() > 1 { set_download_folder(&options[1]); }
    else { set_download_folder(&String::from("HOME")); }
    loop {
        match Clipboard::new() {
            Ok(mut clip) => {
                match clip.get_text() {
                    Ok(new) => download_video(&new, &mut links),
                    Err(e) => on_error(e, "get_text")
                }
            },
            Err(e) => on_error(e, "Clipboard::new")
        }
        thread::sleep(Duration::from_millis(1000));
    }
}
