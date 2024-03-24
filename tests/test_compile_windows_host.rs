#![cfg(target_os = "windows")]
use libpng_src::compile_lib;

use std::assert;

mod temp_dir_helper;
use temp_dir_helper::TempDirHelper;

#[cfg(target_arch = "x86_64")]
#[test]
fn test_compile_x86_64() {
    test_target("x86_64-pc-windows-msvs")
}

#[cfg(target_arch = "aarch64")]
#[test]
fn test_compile_aarch64() {
    test_target("aarch64-pc-windows-msvs")
}

fn test_target(target: &str) {
    let temp_helper = TempDirHelper::new();

    let art_path = compile_lib(target, &temp_helper.temp_dir()).unwrap();
    assert!(art_path.is_file());
}
