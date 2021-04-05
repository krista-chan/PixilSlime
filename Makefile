main:
	cargo build --release && echo "File built to target/release/PixilSlime"

.PHONY: run
run:
	cargo run --release