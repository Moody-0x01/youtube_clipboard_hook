use std::process::exit;

#[allow(unused_imports)]
use std::fs::{exists, create_dir};

#[allow(deprecated)]
use std::env::{home_dir, set_current_dir};
const DOWNLOAD_DIR: &str = "hookclip_deamon";

pub fn set_download_folder(folder: &String)
{
    let default_path;
    let home;

    if folder == "HOME"
    {
        #[allow(deprecated)]
        match home_dir() {
            Some(path) => home = path,
            None => {
                eprintln!("Impossible to get your home dir!");
                exit(1);
            }
        }
        default_path = home.join(DOWNLOAD_DIR);
        let fldr = default_path.to_str().unwrap().to_string();
        set_download_folder(&fldr);
    } else {
        if !exists(folder).unwrap()
        {
            println!("Creating {:#?} as the folder to use for videos", folder);
            create_dir(folder).unwrap();
        }
        set_current_dir(folder).unwrap();
    }
}
