# snake-rust

Snake game coded in rust

![Latest Stable Version](https://github.com/martinf08/snake-rust/workflows/build-ubuntu/badge.svg)
![Latest Stable Version](https://github.com/martinf08/snake-rust/workflows/build-macos/badge.svg)
![Latest Stable Version](https://github.com/martinf08/snake-rust/workflows/build-windows/badge.svg)

## Run the project
Use a stable Rust toolchain. Run from the repository root so `Config.toml`
and `assets/lcd-solid.ttf` can be found.
The graphics dependency targets [Piston Window 0.132](https://docs.rs/piston_window/0.132.0/piston_window/),
retaining the original OpenGL renderer while updating its window backend for
modern platforms (including Apple Silicon).
<pre>cargo run</pre>

## Controls
- Arrow keys: steer (reversing directly is not allowed).
- Space: pause / resume.
- R: restart, preserving the best score and counting a death.
- Escape: quit.

The game restarts automatically after a collision. Filling the board stops the
simulation; press R to play again. The window title shows pause and completion.

## Validation
<pre>cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets</pre>

Configuration errors are reported before opening the window. Board length must
be 6–256 cells, block size 1–1024 pixels, and total width at most 8192 pixels.
`level` must be positive and `block_size * level` must not exceed 600, so the
smooth movement cannot skip an entire cell in one simulation tick. As in the
original, increasing `level` increases speed.

## Optimized build
<pre>cargo build --release</pre>

- Create new folder
- Copy assets folder, Config.toml and binary files in this folder
<pre>
.
├── assets
   └── lcd-solid.ttf   # Font file
├── snake-rust         # Binary file from target/release
└── Config.toml        # Config file
</pre>

## Wall configuration
- solid : The snake die when he touch the wall
- fluid : The snake teleport at the opposite side when he touch the wall

## Mode configuration
- default : The default mode, the board is empty
- portal : Portals blue and yellow appear to wrap the snake when he touch the gate
