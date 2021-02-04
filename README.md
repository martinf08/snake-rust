# snake-rust

Snake game coded in rust

![Latest Stable Version](https://github.com/martinf08/snake-rust/workflows/build/badge.svg)
![version](https://img.shields.io/badge/cargo-1.47.0-blue)
![version](https://img.shields.io/badge/rustc-1.47.0-blue)

## Run the project
<pre>cargo run</pre>

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