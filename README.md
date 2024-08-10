Srcery convert
==============

Attempt at converting an image palette to Srcery, this is very early stages and
is very much a **work in progress**.

## Requirements

Requires rust, cargo

## Usage

```sh
cargo run -- --help
```
```
Convert input image to srcery colorscheme

Usage: srcery-convert <INPUT> <OUTPUT>

Arguments:
  <INPUT>
  <OUTPUT>

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Build a release build to improve speed:
```sh
cargo build --release
```
Then run:
```sh
# binary
target/release/srcery-convert -- input.png output.png
```

## License

[MIT](LICENSE)

