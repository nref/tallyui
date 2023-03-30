build:
	trunk build --release

serve:
	trunk serve --address 0.0.0.0 --port 8080

setup:
	rustup target add wasm32-unknown-unknown
	cargo install trunk

test:
	cargo test --verbose
