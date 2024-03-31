# libpng-src
Helper Cargo package for compiling [libpng](https://github.com/pnggroup/libpng) into a static C library. Meant to be used as build dependency for dufferent `-sys` or `-vendored` packages. Does not provide directly usable `libpng` functionality or bindings.

## Provided version
Compiles `libpng` with version `1.6.43`. Original source code with the license is provided in `libpng` folder without any modifications.

## Currenlly supported OS and targets
Expected to work for:
* Linux: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` (no cross-compilation supported yet)
* Windows: `x86_64-pc-windows-msvs`, `aarch644-pc-windows-msvs` (no cross-compilation supported yet)
* macOS: `x86_64-apple-darwin`, `aarch64-apple-darwin`
* iOS, including simulators (cross-compilation from macOS host): `x86_64-apple-ios`, `aarch64-apple-ios`, `aarch64-apple-ios-sim`

Tested before upload for all the targets, except `aarch64-unknown-linux-gnu` and `aarch64-pc-windows-msvs`.

## Dependenencies for hosts
Doesn't depend on any cargo packages for compilation.
Uses CMake and  `libpng` provided shell scripts. Depends on `zlib` library headers for compilation and dynamic library artifact for testing.

### Linux
* CMake - install via https://cmake.org/ or a package manager
* make - https://www.gnu.org/software/make/
* CC - a C compiler, added to system path
* `zlib` - https://packages.debian.org/bullseye/zlib1g-dev

## Windows
* CMake - vendored by MS Visual Studio;
* MS Visual Studio - https://visualstudio.microsoft.com/
* `zlib` headers and tests helping DLL - vendored in the package

Invoke Cargo via Developer Powershell or Developer Command Prompt for correct work.

### macOS
* CMake - install via https://cmake.org/ or [Homebrew](https://brew.sh/);
* make - vendored by OS
* Xcode - https://developer.apple.com/xcode/
* `zlib` - vendored by OS

### Testing
One of the unit tests invokes CTest (part of CMake) under the hood for testing native code.

## TODO
* Support cross-compilation for Android;

## Authors
**Rust code and scripts:** Alexandr (Alex) Lambov <alex.lambov.md@gmail.com>, &copy; 2024

**libpng** -  see http://www.libpng.org/pub/png/libpng.html
