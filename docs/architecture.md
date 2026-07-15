# Architecture

Overview
- The core is a Rust library/daemon that monitors the clipboard and dispatches downloads.
- There are two CLI binaries under `src/bin/`: a CLI (`cli.rs`) and a daemon (`daemon.rs`).
- A lightweight Python web UI lives in `app/` for optional local management.
- Systemd unit files are provided in `service/` to run the daemon as a background service.

Components
- `src/monitor.rs` — clipboard monitoring and link detection.
- `src/downloader.rs` — download orchestration and adapter to external tools (`yt-dlp`, `wget`, `transmission-cli`).
- `src/spawner.rs` — spawns worker processes and manages concurrency.
- `src/config.rs` & `config/config.json` — runtime configuration and defaults.
- `app/` — optional web UI; `requirements.txt` lists Python deps.

Data flow
1. Clipboard monitor detects a copied URL.
2. URL is validated and matched against known handlers (YouTube, direct media, torrent).
3. The downloader spawns the appropriate external tool to perform the download and logs progress.
4. Completed downloads are placed in the configured `DOWNLOAD_PATH`.

Extending
- To add a new handler, implement detection in `monitor.rs` and a download adapter in `downloader.rs`.