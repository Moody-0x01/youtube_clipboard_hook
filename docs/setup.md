# Setup & Installation

1) Rust build (recommended)

```sh
rustup toolchain install stable
cargo build --release
```

The release binary will be in `target/release/` (e.g. `clippy_daemon`, `cli`).

2) Python web UI (optional)

```sh
python3 -m venv .venv
source .venv/bin/activate
pip install -r app/requirements.txt
python app/main.py
```

3) Systemd service (optional)

Copy the desired unit file from `service/` to `/etc/systemd/system/`, then enable and start:

```sh
sudo cp service/clippy_hook.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now clippy_hook.service
```

4) Dependencies
- Install `yt-dlp`, `ffmpeg`, `wget`, `transmission-cli` using your package manager.

Notes
- On X11 systems install `libx11` for clipboard monitoring. On Wayland, a different backend may be required.