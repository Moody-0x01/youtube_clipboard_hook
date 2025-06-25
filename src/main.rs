use std::{thread, time::Duration};
use std::process::exit;
#[allow(unused_imports)]
use std::fs::{exists, create_dir};
use arboard::{Clipboard};
#[allow(deprecated)]
use std::env::{home_dir, set_current_dir};
use crate::downloader::download_video;
use crate::error_handlers::on_error;
use crate::consts::DOWNLOAD_DIR;
pub mod error_handlers;
pub mod consts;
pub mod downloader;

fn main() {
    let mut links: Vec<String> = Vec::new();
    let dpath;
    let home;

    #[allow(deprecated)]
    match home_dir() {
        Some(path) => home = path,
        None => {
            eprintln!("Impossible to get your home dir!");
            exit(1);
        }
    }
    dpath = home.join(DOWNLOAD_DIR);
    if !exists(&dpath).unwrap()
    {
        println!("Creating {:#?} as the folder to use for videos", dpath);
        create_dir(&dpath).unwrap();
    }
    set_current_dir(&dpath).unwrap();
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
