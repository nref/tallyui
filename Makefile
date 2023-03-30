build:
	trunk build --release

serve:
	trunk serve

setup:
	rustup target add wasm32-unknown-unknown
	cargo install trunk


