use std::{thread, time::Duration};
use arboard::{Clipboard};
use clippy_hook::folder_settings::set_download_folder;
use clippy_hook::downloader::download;
use clippy_hook::error_handlers::on_error;
use clippy_hook::options as opt;
use std::env::args;

fn main()
{
    let mut links: Vec<String> = Vec::new();
    let opts: Vec<String> = args().collect(); 
    let mut options: opt::Options = opt::Options::new();

    if options.parse_options(&opts) == 0 {
        return ;
    }
    set_download_folder(&options.download_path);
    println!("download_path: {}", options.download_path);
    loop {
        match Clipboard::new() {
            Ok(mut clip) => {
                match clip.get_text() {
                    Ok(new) => download(&new, &mut links, &options),
                    Err(e) => on_error(e, "get_text")
                }
            },
            Err(e) => on_error(e, "Clipboard::new")
        }
        thread::sleep(Duration::from_millis(1000));
    }
}
