#![cfg(target_os = "linux")]
use libpng_src::{build_all_artifacts, compile_lib};

use std::assert;

mod helpers;
use helpers::{test_artifacts_build_and_bindgen, test_compile};

#[cfg(target_arch = "x86_64")]
#[test]
fn test_compile_x86_64() {
    test_compile("x86_64-unknown-linux-gnu")
}

#[cfg(target_arch = "aarch64")]
#[test]
fn test_compile_aarch64() {
    test_compile("aarch64-unknown-linux-gnu")
}

#[cfg(target_arch = "x86_64")]
#[test]
fn test_build_and_bindgen_x86_64() {
    test_artifact_build("x86_64-unknown-linux-gnu")
}

#[cfg(target_arch = "aarch64")]
#[test]
fn test_build_and_bindgen_x86_64() {
    test_artifact_build("aarch64-unknown-linux-gnu")
}
