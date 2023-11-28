# run in development mode
run:
	cargo run --features bevy/dynamic_linking

build:
	cargo build --release

run_web:
	cargo run --target wasm32-unknown-unknown

build_web:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir ./out/ --target web ./target/
