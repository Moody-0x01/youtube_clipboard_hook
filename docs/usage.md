# Usage & Examples

Basic CLI

```sh
# run with default download path
cargo run --release -- /path/to/downloads

# or run the built CLI
target/release/cli /path/to/downloads
```

Options
- See `--help` output for the CLI. Typical flags include:
  - `--use_youtube` to prefer `yt-dlp` for YouTube links
  - `--use_wget` to use `wget` for direct media links
  - `--fmts` to restrict downloading to specific file extensions (e.g. `.mp3 .mp4`)

Examples

1) Download only audio formats:

```sh
target/release/cli /downloads --fmts ".mp3 .m4a"
```

2) Quiet mode (less terminal output):

```sh
target/release/cli /downloads --quiet
```

Troubleshooting
- If clipboard monitoring doesn't detect copies, ensure the environment is X11 or the correct clipboard backend is configured.
- Check logs from the systemd unit with `journalctl -u clippy_hook.service`.