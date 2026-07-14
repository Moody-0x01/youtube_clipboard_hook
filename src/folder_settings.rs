use std::process::exit;

#[allow(unused_imports)]
use std::fs::{create_dir, exists};

#[allow(deprecated)]
use std::env::{home_dir, set_current_dir};
const DOWNLOAD_DIR: &str = "Downloads";

pub fn set_download_folder(folder: &mut String) {
    let default_path;
    let home;

    if folder == "DEFAULT" {
        #[allow(deprecated)]
        match home_dir() {
            Some(path) => home = path,
            None => {
                eprintln!("Impossible to get your home dir!");
                exit(1);
            }
        }
        default_path = home.join(DOWNLOAD_DIR);
        let mut fldr = default_path.to_str().unwrap().to_string();
        set_download_folder(&mut fldr);
        *folder = fldr.clone();
    } else {
        if !exists(folder.clone()).unwrap() {
            println!("Creating {:#?} as the folder to use for videos", folder);
            create_dir(folder.clone()).unwrap();
        }
        
        set_current_dir(folder).unwrap();
    }
}
