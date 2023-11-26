# run in development mode
run:
	# flags required to workaround https://github.com/bevyengine/bevy/issues/10524
	cargo run --features bevy/dynamic_linking

build:
	cargo build --release
