use std::{
    env::temp_dir,
    fs::{copy, create_dir_all, remove_dir_all},
};

use super::*;

#[test]
fn test_source_path() {
    let path = source_path();
    assert!(path.exists());
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_execute_command_ok() -> Result<(), Box<dyn Error>> {
    execute("echo", &[OsString::from("test")], &temp_dir())
}

#[cfg(target_os = "windows")]
#[test]
fn test_execute_command_ok() -> Result<(), Box<dyn Error>> {
    execute("systeminfo", &[], &temp_dir())
}

#[test]
fn test_execute_command_fail() {
    assert!(execute("ls", &[OsString::from("non-existent-dir")], &temp_dir()).is_err())
}

#[test]
fn test_native() -> Result<(), Box<dyn Error>> {
    let tmp_dir = temp_dir().join("libpng-sys-test");
    create_dir_all(&tmp_dir)?;

    let source_path = source_path();

    let mut cmake_args = vec![
        source_path.into_os_string(),
        OsString::from("-DPNG_TESTS=ON"),
    ];

    if cfg!(target_os = "windows") {
        let zlib_include_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("win-zlib-include");
        let zlib_lib_path = zlib_include_path.join("zlib.lib");

        let mut include_param = OsString::from("-DZLIB_INCLUDE_DIR=");
        include_param.push(&zlib_include_path);

        let mut lib_param = OsString::from("-DZLIB_LIBRARY=");
        lib_param.push(zlib_lib_path);

        cmake_args.push(include_param);
        cmake_args.push(lib_param);

        copy(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("win-zlib-test-helper/zlib.dll"),
            tmp_dir.join("zlib.dll"),
        )?;
    }

    execute("cmake", &cmake_args, &tmp_dir)?;
    execute(
        "cmake",
        &["--build", ".", "--config", "Debug"].map(OsString::from),
        &tmp_dir,
    )?;
    execute("ctest", &["-C", "Debug"].map(OsString::from), &tmp_dir)?;

    remove_dir_all(&tmp_dir)?;

    Ok(())
}

#[test]
fn test_link_name() {
    let assert_combination = |file_name: &str, expectation: &str| {
        assert_eq!(link_name(file_name.to_string()), expectation.to_string())
    };

    if cfg!(target_os = "windows") {
        assert_combination("libpng.lib", "libpng");
        assert_combination("libpng16.lib", "libpng16");
        assert_combination("libpng_static.lib", "libpng_static");
        assert_combination("libpng16_static.lib", "libpng16_static");
    } else {
        assert_combination("libpng.a", "png");
        assert_combination("libpng.16.a", "png");
        assert_combination("libpng16.a", "png16");
    }
}
