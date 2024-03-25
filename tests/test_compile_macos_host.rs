#![cfg(target_os = "macos")]

mod helpers;
use helpers::{test_artifact_build, test_compile};

#[test]
fn test_compile_for_macos_intel() {
    test_compile("x86_64-apple-darwin")
}

#[test]
fn test_compile_for_macos_arm() {
    test_compile("aarch64-apple-darwin")
}

#[test]
fn test_compile_for_ios_arm() {
    test_compile("aarch64-apple-ios")
}

#[test]
fn test_compile_for_ios_arm_sim() {
    test_compile("aarch64-apple-ios-sim")
}

#[test]
fn test_compile_to_ios_intel_sim() {
    test_compile("x86_64-apple-ios")
}

#[test]
fn test_build_for_macos_intel() {
    test_artifact_build("x86_64-apple-darwin")
}

#[test]
fn test_build_for_macos_arm() {
    test_artifact_build("aarch64-apple-darwin")
}

#[test]
fn test_build_for_ios_arm() {
    test_artifact_build("aarch64-apple-ios")
}

#[test]
fn test_build_for_ios_arm_sim() {
    test_artifact_build("aarch64-apple-ios-sim")
}

#[test]
fn test_build_to_ios_intel_sim() {
    test_artifact_build("x86_64-apple-ios")
}
