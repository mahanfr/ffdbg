
INSTALL_PATH = /usr/bin

.PHONY: build install uninstall

build:
	cargo build --release

install: build
	sudo cp -v ./target/release/ffdbg $(INSTALL_PATH)/ffdbg

uninstall:
	sudo rm -v $(INSTALL_PATH)/ffdbg
