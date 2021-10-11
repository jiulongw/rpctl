# Raspberry Pi ConTroL
A personal Rust learning project that turns on and off my bench light via
Raspberry Pi GPIO by responding to HTTP requests.

The purpose of this project is to learn Rust. Things are built from scratch as
much as possible.

## Build
ARM cross-build toolchain is required. Both `arm-unknown-linux-musleabi` and
`arm-unknown-linux-gnueabi` should work. However, Rust compiler requires `gcc`
to link `arm-unknown-linux-gnueabi` target, and there is no easy-to-get
pre-built one available for macOS. `arm-unknown-linux-musleabi` target on the
other hand, can be linked by `arm-linux-gnueabihf-ld` which is available in
Homebrew.

```sh
rustup target add arm-unknown-linux-musleabi
brew install arm-linux-gnueabi-binutils
```

Once toolchain is ready, use `cargo build`.
