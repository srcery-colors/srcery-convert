Srcery convert
==============

Attempt at converting an image palette to Srcery, this is very early stages and
is very much a **work in progress**.

## Requirements

Requires rust, cargo

## Usage
```sh
cargo build --release
```
Then run:
```sh
# binary
target/release/srcery-convert -- input.png output.png

# cargo
cargo run --release -- input.png output.png
```
