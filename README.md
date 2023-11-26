# README

## Deltas

Initial pass was built following:
https://caballerocoll.com/blog/bevy-rhythm-game/

Changes from that
- Bevy 0.4 -> 0.12
- Rust nightly version = `1.76.0`
  - actually, had to workaround a segfault on mac: `rustup toolchain install nightly-2023-10-18`
  - https://github.com/bevyengine/bevy/issues/10524
- for fast builds, do not need to install `lld` directly.. just `brew install llvm`
  - https://github.com/bevyengine/bevy/blob/main/.cargo/config_fast_builds

