SOURCES := src/bitmasks.rs \
	src/core.rs \
	src/instructions.rs \
	src/main.rs \
	src/opcodes.rs \
	src/opcodesv2.rs \
	src/ophandlers.rs

.PHONY: all
all: $(SOURCES) fmt
	cargo check

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: update
update:
	touch src/*

.PHONY: force
force: update all

.PHONY: build
build:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: flight
flight: fmt
	RUST_LOG=debug cargo run roms/games/Space\ Flight.ch8

.PHONY: lint
lint:
	cargo clippy

%:
	$(MAKE) build
	RUST_LOG=trace cargo run roms/games/$@.ch8

test:
	RUST_LOG=trace cargo test -- --nocapture
