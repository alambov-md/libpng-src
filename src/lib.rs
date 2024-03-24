use std::{
    env::consts::{ARCH as HOST_ARCH, OS as HOST_OS},
    error::Error,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
    vec::Vec,
};

/// Version of the `libpng` library
pub const LIBPNG_VERSION: &str = "1.6.43";

/// Returns the path to the source directory.
/// Use it to generate bindings to the `libpng` if needed.
pub fn source_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("libpng")
}

/// Statically compiles `libpng` library and returns the path to the compiled artifact.
pub fn compile_lib(target_str: &str, working_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    if !allowed_targets_for_host().contains(&target_str) {
        return Err(format!(
            "Unsupported target: {target_str}, for host OS: {HOST_OS}, arch: {HOST_ARCH}"
        )
        .into());
    }

    if working_dir.exists() {
        fs::remove_dir_all(working_dir)?;
    }
    fs::create_dir_all(working_dir)?;

    let source_path = source_path();

    let mut cmake_args = cmake_options(target_str)?;
    cmake_args.push(source_path.into_os_string());

    execute("cmake", &cmake_args, working_dir)?;
    execute(
        "cmake",
        &["--build", ".", "--config", "Release"].map(OsString::from),
        working_dir,
    )?;

    artifact_path(working_dir)
}

fn allowed_targets_for_host() -> Vec<&'static str> {
    match (HOST_OS, HOST_ARCH) {
        ("macos", _) => vec![
            "aarch64-apple-darwin",
            "x86_64-apple-darwin",
            "aarch64-apple-ios",
            "aarch64-apple-ios-sim",
            "x86_64-apple-ios",
        ],
        ("linux", "x86_64") => vec!["x86_64-unknown-linux-gnu"],
        ("linux", "aarch64") => vec!["aarch64-unknown-linux-gnu"],
        ("windows", "x86_64") => vec!["x86_64-pc-windows-msvs"],
        ("windows", "aarch64") => vec!["aarch64-pc-windows-msvs"],
        _ => vec![],
    }
}

fn cmake_options(target_str: &str) -> Result<Vec<OsString>, Box<dyn Error>> {
    match HOST_OS {
        "macos" => macos_cmake_options(target_str),
        "windows" => windows_cmake_options(),
        "linux" => Ok(vec![]),
        _ => Err(format!("Unsupported host OS: {}", HOST_OS).into()),
    }
}

fn macos_cmake_options(target_str: &str) -> Result<Vec<OsString>, Box<dyn Error>> {
    match target_str {
        "aarch64-apple-darwin" => Ok(vec!["-DCMAKE_OSX_ARCHITECTURES=arm64"]),
        "x86_64-apple-darwin" => Ok(vec!["-DCMAKE_OSX_ARCHITECTURES=x86_64"]),
        "aarch64-apple-ios" => Ok(vec![
            "-DCMAKE_SYSTEM_NAME=iOS",
            "-DCMAKE_OSX_ARCHITECTURES=arm64",
        ]),
        "aarch64-apple-ios-sim" => Ok(vec![
            "-DCMAKE_SYSTEM_NAME=iOS",
            "-DCMAKE_OSX_ARCHITECTURES=arm64",
            "-DCMAKE_OSX_SYSROOT=iphonesimulator",
        ]),
        "x86_64-apple-ios" => Ok(vec![
            "-DCMAKE_SYSTEM_NAME=iOS",
            "-DCMAKE_OSX_ARCHITECTURES=x86_64",
            "-DCMAKE_OSX_SYSROOT=iphonesimulator",
        ]),
        _ => Err(format!(
            "Unsupported target: {}, for host OS: {}",
            target_str, HOST_OS
        )
        .into()),
    }
    .map(|str_vec| str_vec.into_iter().map(OsString::from).collect())
}

fn windows_cmake_options() -> Result<Vec<OsString>, Box<dyn Error>> {
    let zlib_include_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("win-zlib-include");
    let zlib_lib_path = zlib_include_path.join("zlib.lib");

    let mut include_param = OsString::from("-DZLIB_INCLUDE_DIR=");
    include_param.push(zlib_include_path);

    let mut lib_param = OsString::from("-DZLIB_LIBRARY=");
    lib_param.push(zlib_lib_path);

    Ok(vec![
        include_param,
        lib_param,
        OsString::from("-DPNG_SHARED=OFF"),
        OsString::from("-DPNG_TESTS=OFF"),
    ])
}

fn artifact_path(working_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let filename = match HOST_OS {
        "windows" => "Release\\libpng16_static.lib",
        _ => "libpng16.a",
    };

    let artifact_path = working_dir.join(filename);

    if !artifact_path.exists() {
        return Err(format!("Artifact not found at path: {}", artifact_path.display()).into());
    }

    Ok(artifact_path)
}

fn execute(command: &str, args: &[OsString], cwd: &Path) -> Result<(), Box<dyn Error>> {
    let output = Command::new(command).current_dir(cwd).args(args).output()?;

    if !output.status.success() {
        let message = format!(
            "Command '{}' failed with status code {}\nError: {}",
            command,
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(message.into());
    }

    let args_vec: Vec<&str> = args
        .iter()
        .map(|a| a.to_str().unwrap_or("!error!"))
        .collect();

    println!("Executed '{} {}' successfully", command, args_vec.join(" "));
    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        env::temp_dir,
        fs::{copy, create_dir_all, read_dir, remove_dir_all},
        io,
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

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_native() -> Result<(), Box<dyn Error>> {
        let tmp_dir = temp_dir().join("libpng-sys-test");

        copy_dir_all(source_path(), &tmp_dir)?;

        execute("./configure", &[], &tmp_dir)?;
        execute("make", &["test"], &tmp_dir)?;

        remove_dir_all(&tmp_dir)?;

        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_native() -> Result<(), Box<dyn Error>> {
        let tmp_dir = temp_dir().join("libpng-sys-test");
        create_dir_all(&tmp_dir)?;

        let source_path = source_path();
        let zlib_include_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("win-zlib-include");
        let zlib_lib_path = zlib_include_path.join("zlib.lib");

        let mut include_param = OsString::from("-DZLIB_INCLUDE_DIR=");
        include_param.push(&zlib_include_path);

        let mut lib_param = OsString::from("-DZLIB_LIBRARY=");
        lib_param.push(zlib_lib_path);

        let cmake_args = [
            source_path.into_os_string(),
            include_param,
            lib_param,
            OsString::from("-DPNG_TESTS=ON"),
        ];

        execute("cmake", &cmake_args, &tmp_dir)?;
        execute(
            "cmake",
            &["--build", ".", "--config", "Release"].map(OsString::from),
            &tmp_dir,
        )?;

        copy(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("win-zlib-test-helper/zlib.dll"),
            tmp_dir.join("zlib.dll"),
        )?;
        execute("ctest", &["-C", "Release"].map(OsString::from), &tmp_dir)?;

        remove_dir_all(&tmp_dir)?;

        Ok(())
    }

    #[allow(dead_code)]
    fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
        create_dir_all(&dst)?;
        for entry in read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }
}
