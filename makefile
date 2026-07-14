
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

all: $(TARGET) app

app: $(APP_DIR)
	python3 -m venv app/venv	
	app/venv/bin/pip install -r ./app/requirements.txt
	cp -p scripts/clippy_app $(INSTALL_DIRECTORY)
	cp -r app/* "$(APP_DIR)"
	
$(APP_DIR):
	mkdir -p $(APP_DIR)

$(CONFIG_DIR):
	mkdir -p $(CONFIG_DIR)

$(TARGET): $(SRC) $(CONFIG_DIR)
	cargo build --release

$(INSTALL_DIRECTORY):
	mkdir -p $(INSTALL_DIRECTORY)

$(SERVICES_DIR):
	mkdir -p $(SERVICES_DIR)

install: $(INSTALL_DIRECTORY) $(SERVICES_DIR) $(CONFIG_DIR) $(TARGET) app
	cp $(TARGET) $(INSTALL_DIRECTORY)
	cp $(CLI)    $(INSTALL_DIRECTORY)
	cp $(SERVICES) $(SERVICES_DIR)
	cp $(CONFIG) $(CONFIG_DIR)

run: install
	$(SYS_CTL) daemon-reload
	$(SYS_CTL) start clippy_hook.service
	$(SYS_CTL) start clippy_configure.service

clean:
	cargo clean

re: clean all

.PHONY: re clean $(TARGET) run install $(SERVICES_DIR) $(INSTALL_DIRECTORY)
