use std::{
    env::consts::{OS as HOST_OS, ARCH as HOST_ARCH},
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};

/// Version of the `libpng` library
pub const LIBPNG_VERSION: &str = "1.6.43";

/// Returns the path to the sorce directory.
/// Use it to generate bindings to the `libpng` if needed.
pub fn source_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("libpng")
}

/// Statically compiles `libpng` library and returns the path to the compiled artifact.
pub fn compile_lib(target_str: &str, working_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    if !allowed_targets_for_host().contains(&target_str) {
        return Err(format!("Unsupported target: {target_str}, for host OS: {HOST_OS}, arch: {HOST_ARCH}").into());
    }

    let source_path = source_path();
    let source_path_str = source_path.as_os_str().to_str().unwrap();

    let mut cmake_args = cmake_options(target_str)?;
    cmake_args.push(source_path_str);

    cmake_setup(working_dir, &cmake_args)?;
    make(&working_dir)?;
    artifact_path(&working_dir)
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
        _ => vec![],
    }
}

fn cmake_options(target_str: &str) -> Result<Vec<&str>, Box<dyn Error>> {
    match HOST_OS {
        "macos" => macos_cmake_options(target_str),
        "linux" => Ok(vec![]),
        _ => Err(format!("Unsupported host OS: {}", HOST_OS).into()),
    }
}

fn macos_cmake_options(target_str: &str) -> Result<Vec<&str>, Box<dyn Error>> {
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
}

fn cmake_setup(working_dir: &Path, args: &[&str]) -> Result<(), Box<dyn Error>> {
    if !working_dir.exists() {
        std::fs::create_dir_all(&working_dir)?;
    }

    execute("cmake", args, working_dir)
}

fn make(working_dir: &Path) -> Result<(), Box<dyn Error>> {
    execute("make", &[], working_dir)
}

fn artifact_path(working_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let filename = match HOST_OS {
        "macos" => "libpng.a",
        _ => "libpng.a",
    };

    let artifact_path = working_dir.join(filename);

    if !artifact_path.exists() {
        return Err(format!("Artifact not found at path: {}", artifact_path.display()).into());
    }

    Ok(artifact_path)
}

fn execute(command: &str, args: &[&str], cwd: &Path) -> Result<(), Box<dyn Error>> {
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

    println!("Executed '{} {}' successfully", command, args.join(" "));
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

    #[test]
    fn test_execute_command_ok() -> Result<(), Box<dyn Error>> {
        execute("echo", &["hello"], &temp_dir())
    }

    #[test]
    fn test_execute_command_fail() {
        assert!(execute("ls", &["non-existent-dir"], &temp_dir()).is_err())
    }

    #[test]
    fn test_native() -> Result<(), Box<dyn Error>> {
        let tmp_dir = temp_dir().join("libpng-sys-test");

        copy_dir_all(source_path(), &tmp_dir)?;

        execute("./configure", &[], &tmp_dir)?;
        execute("make", &["test"], &tmp_dir)?;

        remove_dir_all(&tmp_dir)?;

        Ok(())
    }

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
