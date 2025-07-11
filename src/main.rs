use std::{thread, time::Duration};
use arboard::{Clipboard};
use crate::folder_settings::set_download_folder;
use crate::downloader::download_video;
use crate::error_handlers::on_error;
use crate::options as opt;
use std::env::args;

pub mod options;
pub mod error_handlers;
pub mod downloader;
pub mod folder_settings;


fn main()
{
    let mut links: Vec<String> = Vec::new();
    let opts: Vec<String> = args().collect(); 
    let mut options: opt::Options = opt::Options::new();

    if options.parse_options(&opts) == 0 {
        return ;
    }
    // options.log();
    set_download_folder(&options.download_path);
    loop {
        match Clipboard::new() {
            Ok(mut clip) => {
                match clip.get_text() {
                    Ok(new) => download_video(&new, &mut links, &options),
                    Err(e) => on_error(e, "get_text")
                }
            },
            Err(e) => on_error(e, "Clipboard::new")
        }
        thread::sleep(Duration::from_millis(1000));
    }
}
