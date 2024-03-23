use std::{
    env::temp_dir,
    fs::{create_dir_all, remove_dir_all},
    path::PathBuf,
    time::SystemTime,
};

#[derive(Clone)]
pub struct TempDirHelper {
    temp_dir: PathBuf,
}

impl TempDirHelper {
    pub fn new() -> Self {
        let millis = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let random_postfix: u64 = rand::random();

        let temp_dir = temp_dir().join(format!("libpng-src-test-{millis}-{random_postfix}"));

        create_dir_all(&temp_dir).expect("Cannot create temp dir");

        Self { temp_dir }
    }

    pub fn temp_dir(&self) -> PathBuf {
        self.temp_dir.clone()
    }
}

impl Drop for TempDirHelper {
    fn drop(&mut self) {
        remove_dir_all(&self.temp_dir).unwrap();
    }
}
