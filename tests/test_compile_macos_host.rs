#![cfg(target_os = "macos")]
use libpng_src::compile_lib;

use std::assert;

mod temp_dir_helper;
use temp_dir_helper::TempDirHelper;

#[test]
fn test_compile_to_macos_intel() {
    test_target("x86_64-apple-darwin")
}

#[test]
fn test_compile_to_macos_arm() {
    test_target("aarch64-apple-darwin")
}

#[test]
fn test_compile_to_ios_arm() {
    test_target("aarch64-apple-ios")
}

#[test]
fn test_compile_to_ios_arm_sim() {
    test_target("aarch64-apple-ios-sim")
}

#[test]
fn test_compile_to_ios_intel_sim() {
    test_target("x86_64-apple-ios")
}

fn test_target(target: &str) {
    let temp_helper = TempDirHelper::new();

    let art_path = compile_lib(target, &temp_helper.temp_dir()).unwrap();
    assert!(art_path.is_file());
}
