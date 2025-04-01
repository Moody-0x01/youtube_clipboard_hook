use std::{thread, time::Duration};
#[allow(unused_imports)]
use std::process::{Command, Stdio};
use std::fs::{exists, create_dir};
use arboard::{Clipboard, Error};
use std::process::exit;
#[allow(deprecated)]
use std::env::{home_dir, set_current_dir};

fn on_error(e: Error, func: &str)
{
    eprintln!("Source: {}", func);
    match e {
    Error::ContentNotAvailable => {
        eprintln!("ContentNotAvailable");
    },
    Error::ClipboardNotSupported => {
        eprintln!("ClipboardNotSupported");
        exit(1);
    },
    Error::ClipboardOccupied => {
        eprintln!("ClipboardOccupied");
    },
    Error::ConversionFailure => {
        eprintln!("ConversionFailure");
    },
    Error::Unknown { description } => {
        eprintln!("Unknown: {}", description);
        exit(1);
    },
    _ => {
        eprintln!("an unexpected error was returned");
        exit(1);
    }
    }
}

const DOWNLOAD_DIR: &str = "hookclip_deamon";

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
        println!("Creating {:#?}", dpath);
        create_dir(&dpath).unwrap();
    }
    set_current_dir(&dpath).unwrap();
    loop {
        match Clipboard::new() {
            Ok(mut clip) => {
                match clip.get_text() {
                    Ok(new) => {
                        if links.contains(&new)
                        {
                            continue ;
                        }
                        if  new.starts_with("https://www.youtube.com/watch?v")
                            || new.starts_with("https://youtu.be") || new.starts_with("https://www.youtube.com/live") {
                            println!("Downloading: [{}]", new);
                            let current_link = new.clone();
                            let child = Command::new("yt-dlp")
                            .arg(&current_link)
                            .stdout(Stdio::null())
                            .spawn()
                            .expect("Failed to execute `yt-dlp` command, make sure u have it installed");
                            thread::spawn(move || {
                                let status = child.wait_with_output().expect("Failed to wait on command");
                                if status.status.success() {
                                    println!("{} was downloaded successfully.", current_link);
                                } else {
                                    println!("Process failed with exit code: {:?}", status.status.code());
                                }
                            });
    
                            links.push(new.clone());
                        }
                    },
                    Err(e) => on_error(e, "get_text")
                }
            },
            Err(e) => on_error(e, "Clipboard::new")
        }
        thread::sleep(Duration::from_millis(100));
    }
}
