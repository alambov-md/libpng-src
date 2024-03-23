use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};

trait SourcePathProvider {
    fn new() -> Self;
    fn source_dir(&self) -> PathBuf;
}

struct DefaultSourcePathProvider;

impl SourcePathProvider for DefaultSourcePathProvider {
    fn new() -> Self {
        DefaultSourcePathProvider {}
    }

    fn source_dir(&self) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("libpng")
    }
}

pub const PNG_LIB_VERSION: &str = "1.6.43";

pub fn png_header_path() -> PathBuf {
    _png_header_path(&DefaultSourcePathProvider::new())
}

pub fn compile_lib() -> Result<PathBuf, Box<dyn Error>> {
    _compile_lib(&DefaultSourcePathProvider::new())
}

pub fn make_clean() -> Result<(), Box<dyn Error>> {
    _make_clean(&DefaultSourcePathProvider::new())
}

fn execute(command: &str, args: &[&str], cwd: &Path) -> Result<String, Box<dyn Error>> {
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

    Ok(String::from_utf8_lossy(&output.stderr).into())
}

fn _png_header_path<T: SourcePathProvider>(provider: &T) -> PathBuf {
    provider.source_dir().join("png.h")
}

fn _compile_lib<T: SourcePathProvider>(provider: &T) -> Result<PathBuf, Box<dyn Error>> {
    let source_dir = provider.source_dir();

    execute("./configure", &[], &source_dir)?;
    execute("make", &[], &source_dir)?;

    let lib_path = source_dir.join(".libs").join("libpng16.a");

    if !lib_path.exists() {
        return Err("'libpng16.a' not found after comnpilation.".into());
    }

    Ok(lib_path)
}

fn _make_clean<T: SourcePathProvider>(provider: &T) -> Result<(), Box<dyn Error>> {
    execute("make", &["clean"], &provider.source_dir()).map(|_| ())
}

#[cfg(test)]
mod tests {
    use std::{
        env::temp_dir,
        fmt::Display,
        fs::{copy, create_dir_all, read_dir, remove_dir_all},
        io,
        time::SystemTime,
    };

    use super::*;

    #[derive(Debug)]
    struct ErrorString(String);

    impl Display for ErrorString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Error for ErrorString {}

    #[derive(Clone)]
    struct TestSourcePathProvider {
        source_dir: PathBuf,
    }

    impl SourcePathProvider for TestSourcePathProvider {
        fn new() -> Self {
            let millis = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let random_postfix: u64 = rand::random();

            let original_dir = DefaultSourcePathProvider::new().source_dir();
            let dest_dir = temp_dir().join(format!("libpng-sys-test-{millis}-{random_postfix}"));

            copy_dir_all(original_dir, &dest_dir).unwrap();

            Self {
                source_dir: dest_dir,
            }
        }

        fn source_dir(&self) -> PathBuf {
            self.source_dir.clone()
        }
    }

    impl Drop for TestSourcePathProvider {
        fn drop(&mut self) {
            remove_dir_all(&self.source_dir).unwrap();
        }
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

    #[test]
    fn test_png_header_path() {
        let path = png_header_path();
        assert!(path.exists());
    }

    #[test]
    fn test_compile_lib() -> Result<(), Box<dyn Error>> {
        let source_provider = TestSourcePathProvider::new();

        let lib_path = _compile_lib(&source_provider)?;
        assert!(lib_path.exists());

        drop(source_provider);
        Ok(())
    }

    #[test]
    fn test_make_clean() -> Result<(), Box<dyn Error>> {
        let source_provider = TestSourcePathProvider::new();
        let source_dir = source_provider.source_dir();

        execute("./configure", &[], &source_dir)?;
        execute("make", &[], &source_dir)?;
        assert!(source_dir.join(".libs").exists());

        _make_clean(&source_provider)?;
        assert!(!source_dir.join(".libs").exists());

        Ok(())
    }

    #[test]
    fn test_native() -> Result<(), Box<dyn Error>> {
        let source_provider = TestSourcePathProvider::new();
        let source_dir = source_provider.source_dir();

        execute("./configure", &[], &source_dir)?;

        execute("make", &["test"], &source_dir).map(|_| ())
    }
}
