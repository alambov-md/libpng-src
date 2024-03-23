# libpng-src
Helper cargo package for compiling [libpng](https://github.com/pnggroup/libpng) into a static C library. Meant to be used as build dependency for dufferent `-sys` packages. Does not provide directly usable `libpng` functionality or bindings.

## Provided version
Compiles `libpng` with version `1.6.43` without modifications. Original source code with license provided in `libpng` folder without any modifications.

## Currenlly supported OS and targets
Expected to work for:
* Linux: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`
* macOS: `x86_64-apple-darwin`, `aarch64-apple-darwin`

Tested before upload for:
* `x86_64-unknown-linux-gnu`
* `aarch64-apple-darwin`

## Dependenencies
Doesn't depend on any cargo packages for compilation.
Uses `make` and `libpng` provided shell scripts. Depends on `make` and `libpng`'s `Makefile`, dependencies plus [zlib1g-dev](https://packages.debian.org/bullseye/zlib1g-dev) for compilation on Linux hosts. On macOS it uses `libz` which by default is present at macOS.

## TODO
* Support cross-compilation for mobile platforms (iOS/Android);
* Support cross-compilation for macOS x86_64 at arm host and vice versa;
* Support Windows;

## Authors
**Rust code and scripts:** Alexandr (Alex) Lambov <alex.lambov.md@gmail.com>, &copy; 2024

**libpng** -  see http://www.libpng.org/pub/png/libpng.html
