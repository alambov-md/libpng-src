# libpng-src
Helper cargo package for compiling [libpng](https://github.com/pnggroup/libpng) into a static C library. Meant to be used as build dependency for dufferent `-sys` packages. Does not provide directly usable `libpng` functionality or bindings.

## Provided version
Compiles `libpng` with version `1.6.43` without modifications. Original source code with license provided in `libpng` folder without any modifications.

## Currenlly supported OS and targets
Expected to work for:
* Linux: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` (no cross-compilation supproted yet)
* macOS: `x86_64-apple-darwin`, `aarch64-apple-darwin`
* iOS, including simulators (cross-compilation from macOS host): `x86_64-apple-ios`, `aarch64-apple-ios`, `aarch64-apple-ios-sim`

Tested before upload for all the targets, except `aarch64-unknown-linux-gnu`

## Compilation dependenencies
Doesn't depend on any cargo packages for compilation.
Uses `cmake` and `make` provided shell scripts. Depends on `make` and `libpng`'s `Makefile`, dependencies plus [zlib1g-dev](https://packages.debian.org/bullseye/zlib1g-dev) for compilation on Linux hosts. On macOS and iOS it uses OS-vendored `libz`.

## TODO
* Support cross-compilation for Android;
* Support Windows;

## Authors
**Rust code and scripts:** Alexandr (Alex) Lambov <alex.lambov.md@gmail.com>, &copy; 2024

**libpng** -  see http://www.libpng.org/pub/png/libpng.html
