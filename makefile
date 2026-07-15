SRC=src/consts.rs src/downloader.rs src/error_handlers.rs src/folder_settings.rs src/bin/cli.rs src/bin/daemon.rs src/options.rs 
TARGET=./target/release/clippy_daemon
CLI=./target/release/clippy_cli
SERVICES=./service/*
SERVICES_DIR=$(HOME)/.config/systemd/user/
CONFIG_DIR=$(HOME)/.config/clippy_hook
CONFIG=./config/config.json
APP_DIR=$(HOME)/.local/share/clippy/
SYS_CTL=systemctl --user 
INSTALL_DIRECTORY=$(HOME)/.local/bin
APP_FILES=app/requirements.txt $(wildcard app/*.py) $(wildcard app/views/*)

all: $(TARGET) app

app: .app_built

.app_built: $(APP_FILES) | $(APP_DIR)
	python3 -m venv app/venv    
	app/venv/bin/pip install -r ./app/requirements.txt
	cp -p scripts/* $(INSTALL_DIRECTORY)
	cp -r app/* "$(APP_DIR)"
	touch .app_built  # Marks the timestamp of the last successful build

$(APP_DIR):
	mkdir -p $(APP_DIR)

$(CONFIG_DIR):
	mkdir -p $(CONFIG_DIR)

$(TARGET): $(SRC) | $(CONFIG_DIR)
	cargo build --release

build: $(TARGET) app

$(INSTALL_DIRECTORY):
	mkdir -p $(INSTALL_DIRECTORY)

$(SERVICES_DIR):
	mkdir -p $(SERVICES_DIR)

install: build $(INSTALL_DIRECTORY) $(SERVICES_DIR) $(CONFIG_DIR)
	cp $(TARGET) $(INSTALL_DIRECTORY)
	cp $(CLI)    $(INSTALL_DIRECTORY)
	cp $(SERVICES) $(SERVICES_DIR)
	cp $(CONFIG) $(CONFIG_DIR)

start:
	systemctl --user daemon-reload
	$(SYS_CTL) start clippy_configure.service
	$(SYS_CTL) start clippy_hook.service

stop:
	$(SYS_CTL) daemon-reload
	$(SYS_CTL) stop clippy_hook.service || true
	$(SYS_CTL) stop clippy_configure.service || true

run: stop install start

clean: stop
	cargo clean
	rm -f .app_built

re: clean all

.PHONY: all app re clean run install start stop
