.PHONY: build install uninstall clean run

PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin
DATADIR = $(PREFIX)/share

build:
	cargo build --release

install: build
	install -Dm755 target/release/driftwm-settings $(DESTDIR)$(BINDIR)/driftwm-settings
	install -Dm644 driftwm-settings.desktop $(DESTDIR)$(DATADIR)/applications/driftwm-settings.desktop

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/driftwm-settings
	rm -f $(DESTDIR)$(DATADIR)/applications/driftwm-settings.desktop

clean:
	cargo clean

run:
	cargo run
