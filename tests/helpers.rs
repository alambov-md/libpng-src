use std::{
    env::temp_dir,
    fs::{create_dir_all, read_dir, remove_dir_all},
    path::{Path, PathBuf},
    time::SystemTime,
};

use libpng_src::{build_artifact, compile_lib};

const HEADER_FILES: [&str; 3] = ["png.h", "pngconf.h", "pnglibconf.h"];

#[derive(Clone)]
struct TempDirHelper {
    temp_dir: PathBuf,
}

impl TempDirHelper {
    fn new() -> Self {
        let millis = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let random_postfix: u64 = rand::random();

        let temp_dir = temp_dir().join(format!("libpng-src-test-{millis}-{random_postfix}"));
        create_dir_all(&temp_dir).expect("Cannot create temp dir");

        Self { temp_dir }
    }

    fn temp_dir(&self) -> PathBuf {
        self.temp_dir.clone()
    }
}

impl Drop for TempDirHelper {
    fn drop(&mut self) {
        remove_dir_all(&self.temp_dir).unwrap();
    }
}

pub fn test_compile(target: &str) {
    let temp_helper = TempDirHelper::new();

    let art_path = compile_lib(target, &temp_helper.temp_dir()).unwrap();
    assert!(art_path.is_file());
}

pub fn test_artifact_build(target: &str) {
    let temp_helper = TempDirHelper::new();

    let artifact_info = build_artifact(target, &temp_helper.temp_dir()).unwrap();

    assert_dir_not_empty(&artifact_info.include_dir);

    for h_file in HEADER_FILES {
        assert!(artifact_info.include_dir.join(h_file).is_file());
    }

    assert!(!artifact_info.link_name.is_empty())
}

fn assert_dir_not_empty(dir_path: &Path) {
    assert!(dir_path.is_dir());

    let mut read_iter = read_dir(dir_path).unwrap();
    assert!(read_iter.next().is_some());
}
