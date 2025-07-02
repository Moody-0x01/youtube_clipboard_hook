use std::process::{Child, Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::{thread};

struct Progress {
    total: usize,
    completed: usize,
}

static PROGRESS: OnceLock<Mutex<Progress>> = OnceLock::new();

fn get_progress() -> &'static Mutex<Progress> {
    PROGRESS.get_or_init(|| Mutex::new(Progress { total: 0, completed: 0 }))
}

fn contains_media_extension(link: &str) -> bool {
    let media_extensions = [
        ".mp3", ".wav", ".flac", ".aac", ".ogg", ".m4a", ".wma",
        ".mp4", ".avi", ".mkv", ".mov", ".wmv", ".flv", ".webm", ".m4v",
        ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".tiff", ".svg", ".webp",
        ".pdf",
    ];
    
    let link_lower = link.to_lowercase();
    media_extensions.iter().any(|&ext| link_lower.ends_with(ext))
}

fn download_using_backend(backend: &str, new: &String)
{
    println!("{}: Downloading: {}", backend, new);
    let current_link = new.clone();
    let backend_ = String::from(backend);
    let mut p = get_progress().lock().unwrap();
    p.total += 1;

    let child: Child = if backend == "wget" {
        Command::new(backend).arg("-q").arg(&current_link).stdout(Stdio::null())
        .spawn()
        .expect(format!("Failed to execute `{}` command, make sure u have it installed", backend).as_str())
    } else {
        Command::new(backend).arg(&current_link).stdout(Stdio::null())
        .spawn()
        .expect(format!("Failed to execute `{}` command, make sure u have it installed", backend).as_str())
    };
    thread::spawn(move || {
        let status = child.wait_with_output().expect("Failed to wait on command");
        let mut p = get_progress().lock().unwrap();
        p.completed += 1;
        if status.status.success() {
            println!("[{}/{}] {}: {} was downloaded successfully", p.completed, p.total, backend_, current_link);
        } else {
            println!("[{}/{}] {}: Process failed with exit code: {:?}", p.completed, p.total, backend_, status.status.code());
        }
    });
}

pub fn download_video(new: &String, links: &mut Vec<String>)
{
    if links.contains(new)
    {
        return ;
    }
    if  new.starts_with("https://www.youtube.com/watch?v")
        || new.starts_with("https://youtu.be") || new.starts_with("https://www.youtube.com/live") {
        download_using_backend("yt-dlp", new);
    }
    else if new.starts_with("https://") && contains_media_extension(new) {
        download_using_backend("wget", new);
    } else {
        return ;
    }
    links.push(new.clone());
    // Add another backend to download torrent files. in the background
}
