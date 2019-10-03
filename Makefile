flight:
	RUST_LOG=trace cargo run roms/games/Space\ Flight.ch8
lint:
	cargo clippy
test:
	RUST_LOG=trace cargo test -- --nocapture
