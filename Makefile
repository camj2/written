PREFIX = /usr/local

all:
	@cargo build --release
	@strip target/release/written

install: target/release/written
	@mkdir -p $(PREFIX)/bin
	@install -m 755 target/release/written $(PREFIX)/bin/written

uninstall:
	@rm -f $(PREFIX)/bin/written

clean:
	@cargo clean

.PHONY: all install uninstall clean
