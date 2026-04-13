.PHONY: build install uninstall clean run

PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin
DATADIR = $(PREFIX)/share

build:
	@command -v cargo >/dev/null 2>&1 || { echo "cargo not found in PATH"; exit 1; }
	cargo build --release

install:
	@test -f target/release/driftwm-settings || { echo "Binary not found. Run 'cargo build --release' first."; exit 1; }
	install -Dm755 target/release/driftwm-settings $(DESTDIR)$(BINDIR)/driftwm-settings
	install -Dm644 driftwm-settings.desktop $(DESTDIR)$(DATADIR)/applications/driftwm-settings.desktop
	install -Dm644 driftwm-settings.svg $(DESTDIR)$(DATADIR)/icons/hicolor/scalable/apps/driftwm-settings.svg

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/driftwm-settings
	rm -f $(DESTDIR)$(DATADIR)/applications/driftwm-settings.desktop
	rm -f $(DESTDIR)$(DATADIR)/icons/hicolor/scalable/apps/driftwm-settings.svg

clean:
	@command -v cargo >/dev/null 2>&1 && cargo clean || rm -rf target

run:
	@command -v cargo >/dev/null 2>&1 || { echo "cargo not found in PATH"; exit 1; }
	cargo run
