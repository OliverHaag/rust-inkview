# rust-inkview
Rust-bindings for the PocketBook InkView library
## Setup
Clone <https://github.com/pocketbook/SDK_6.3.0/tree/5.19> and set it up as described.

Set target and library directory via .cargo/config, replacing `$SDK_DIR` accordingly:
```
[build]
target = "arm-unknown-linux-gnueabi"

[target.arm-unknown-linux-gnueabi]
linker = "$SDK_DIR/SDK-B288/usr/bin/arm-obreey-linux-gnueabi-clang"
rustflags = [
	"-C", "target-cpu=cortex-a7",
	"-L", "$SDK_DIR/SDK-B288/usr/arm-obreey-linux-gnueabi/sysroot/usr/local/lib/"
]
```
Set environment variables for bindgen, replacing `$SDK_DIR` accordingly:
```
$ export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$SDK_DIR/SDK-B288/usr/arm-obreey-linux-gnueabi/sysroot/ -I$SDK_DIR/SDK-B288/usr/arm-obreey-linux-gnueabi/sysroot/usr/include/freetype2"
```
## Building
For build with debugging info:
```
$ cargo build --release
```
For optimized build:
```
$ RUSTFLAGS="-C link-arg=-s" cargo build --release
```
## Using
Add inkview as dependency to your Crate.toml:
```
[dependencies]
inkview = { path = "path/to/inkview", version = "0.1" }
```
