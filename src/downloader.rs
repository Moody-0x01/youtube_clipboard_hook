use std::process::{Child, Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::{thread};
use crate::options::{Options};

struct Progress {
    total: usize,
    completed: usize,
}

// transmission-remote  Host:Port -r $LINK
static TRANSMISSIONHOST: &str = "localhost:6969";
static PROGRESS: OnceLock<Mutex<Progress>> = OnceLock::new();

fn get_progress() -> &'static Mutex<Progress> {
    PROGRESS.get_or_init(|| Mutex::new(Progress { total: 0, completed: 0 }))
}

fn download_using_backend(backend: &str, new: &String, opts: &Options)
{
    let quiet = opts.quiet;
    let current_link = new.clone();
    let backend_ = String::from(backend);
    let mut p = get_progress().lock().unwrap();

    p.total += 1;
    let child: Child = if backend == "wget" {
        Command::new(backend).arg("-q").arg(&current_link).stdout(Stdio::null())
        .spawn()
        .expect(format!("Failed to execute `{}` command, make sure u have it installed", backend).as_str())
    } else if backend == "transmission-remote" {
            Command::new(backend)
            .arg(TRANSMISSIONHOST)
            .arg("-a")
            .arg(&current_link)
            .stdout(Stdio::null())
            .spawn()
            .expect(format!("Failed to execute `{}` command, make sure u have it installed", backend).as_str())
    } else {
        if opts.use_soundcloud
        {
            Command::new("daudio.sh")
            .arg(&current_link)
            .stdout(Stdio::null())
            .spawn()
            .expect(format!("Failed to execute `{}` command, make sure u have it installed", backend).as_str())
        }
        else {
            Command::new(backend)
            .arg(&current_link)
            .stdout(Stdio::null())
            .spawn()
            .expect(format!("Failed to execute `{}` command, make sure u have it installed", backend).as_str())
        }
    };
    if !quiet {
        println!("{}: Downloading: {}", backend, new);
    }
    thread::spawn(move || {
        let status = child.wait_with_output().expect("Failed to wait on command");
        let mut p = get_progress().lock().unwrap();
        if quiet
        {
            return;
        } else if status.status.success() {
            p.completed += 1;
            if backend_ == "transmission-remote" {
                println!("[{}/{}] {}: {} was added successfully", p.completed, p.total, backend_, current_link);
            } else {
                println!("[{}/{}] {}: {} was downloaded successfully", p.completed, p.total, backend_, current_link);
            }
        } else {
            println!("[{}/{}] {}: Process failed with exit code: {:?}", p.completed, p.total, backend_, status.status.code());
        }
    });
}

fn is_link(link: &String) -> bool
{
    let flag =  link.starts_with("https://www.youtube.com/watch?v")
        || link.starts_with("https://youtu.be") 
        || link.starts_with("https://www.youtube.com/live");
    return flag || link.starts_with("https://");
}

pub fn is_magnet_or_torrent(s: &String) -> bool {
    if s.starts_with("magnet:?") {
        return true;
    }
    if s.ends_with(".torrent") {
        return true;
    }

    false
}

pub fn download(new: &String, links: &mut Vec<String>, opts: &Options)
{
    // if !opts.download_path_set {
    //     // Figure out what folder to use based on the extension
    //     todo!();
    // }
    if links.contains(new)
    {
        return ;
    }
    if opts.use_transmission && is_magnet_or_torrent(new) {
        download_using_backend("transmission-remote", new, opts);
    } else if opts.use_mpv && is_link(new) {
        download_using_backend("mpv", new, opts);
    } else if  opts.use_youtube && is_link(new) {
        download_using_backend("yt-dlp", new, opts);
    } else if new.starts_with("https://") && opts.is_fmt_supported(new) && opts.use_wget {
        download_using_backend("wget", new, opts);
    } else {
        return ;
    }
    links.push(new.clone());
}
