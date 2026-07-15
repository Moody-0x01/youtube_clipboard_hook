#!/usr/bin/env bash
set -euo pipefail

HERE=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
ROOT=$(cd "$HERE/.." && pwd)

usage(){
  cat <<EOF
Usage: $0 <command>
Commands:
  build    - build Rust binaries and Python app (runs 'make build')
  install  - install binaries, app, and systemd units (runs 'make install')
  start    - start services (runs 'make start')
  stop     - stop services (runs 'make stop')
  run      - stop, install, start (runs 'make run')
  clean    - stop and clean (runs 'make clean')
  status   - show systemd user service status
  help     - show this help
EOF
}

cmd=${1:-help}
case "$cmd" in
  build)
    make build
    ;;
  install)
    make install
    echo "Installed to ~/.local/bin and ~/.local/share/clippy"
    ;;
  start)
    make start
    ;;
  stop)
    make stop
    ;;
  run)
    make run
    ;;
  clean)
    make clean
    ;;
  status)
    systemctl --user status clippy_hook.service clippy_configure.service || true
    ;;
  help|-h|--help)
    usage
    ;;
  *)
    echo "Unknown command: $cmd"
    usage
    exit 2
    ;;
esac
