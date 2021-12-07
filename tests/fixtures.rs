use std::path::PathBuf;

use tempdir::TempDir;

pub fn temp_directory() -> PathBuf {
    let tmp_dir = TempDir::new("example").unwrap();
    tmp_dir.path().to_path_buf()
}
