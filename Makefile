BINARY_NAME = kaidoku
INSTALL_PATH = /usr/local/bin

.PHONY: all
all: build

.PHONY: build
build:
	cargo build --release

.PHONY: install
install: build
	install -Dm755 target/release/$(BINARY_NAME) $(DESTDIR)$(INSTALL_PATH)/$(BINARY_NAME)

.PHONY: uninstall
uninstall:
	rm -f $(DESTDIR)$(INSTALL_PATH)/$(BINARY_NAME)

.PHONY: clean
clean:
	cargo clean

