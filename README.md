# youtube_clipboard_hook

A small open-source tool (Rust + helpers) that watches the system clipboard for links and automatically downloads recognized media links (YouTube, direct media files, torrents, etc.).

See the full documentation in the `docs/` folder.

Key components
- `src/` - Rust daemon, CLI and library code
- `service/` - example systemd unit files
- `app/` - small Python web UI and static views

Prerequisites
- `cargo` (Rust toolchain) for building the project
- `ffmpeg` (recommended for processing media)
- `libx11` (clipboard monitoring on X11)
- `yt-dlp` (YouTube downloads)
- `wget` (direct media downloads)
- `transmission-cli` (optional, for torrents)

Quickstart
1. Build the Rust project:

```sh
cargo build --release
```

2. Run the CLI (development):

```sh
cargo run --release -- [DOWNLOAD_PATH]
```

3. To run the daemon, install the appropriate unit from the `service/` directory and enable it with systemd.

Documentation
- Architecture: docs/architecture.md
- Setup & install: docs/setup.md
- Usage & examples: docs/usage.md
- Contributing: docs/contributing.md

For development notes and code layout, see the `src/` and `app/` folders.
