# run in development mode
run:
	# flags required to workaround https://github.com/bevyengine/bevy/issues/10524
	RUSTFLAGS='-Zcross-crate-inline-threshold=0' cargo run --features bevy/dynamic_linking

build:
	cargo build --release
