SRC=src/consts.rs src/downloader.rs src/error_handlers.rs src/folder_settings.rs src/main.rs src/options.rs
TARGET=./target/release/cphook
SERVICE=./service/cphook.service
SERVICES_DIR=$(HOME)/.config/systemd/user/
INSTALL_DIRECTORY=~/.local/bin/

all: $(TARGET)

$(TARGET): $(SRC)
	cargo build --release

$(INSTALL_DIRECTORY):
	mkdir -p $(INSTALL_DIRECTORY)

$(SERVICES_DIR):
	mkdir -p $(SERVICES_DIR)

install: $(INSTALL_DIRECTORY)
	cp $(TARGET) $(INSTALL_DIRECTORY)
	cp $(SERVICE) $(SERVICES_DIR)

run: $(TARGET)
	$(TARGET)

clean:
	cargo clean

re: clean all

.PHONY: re clean $(TARGET) run install $(SERVICES_DIR) $(INSTALL_DIRECTORY)
