use std::{
    fs::File,
    path::{Path, PathBuf},
};

use tempdir::TempDir;
use yggdrasil::metadata::{DocumentMetadata, YggdrasilMetadataStore};

pub fn temp_directory() -> PathBuf {
    let tmp_dir = TempDir::new("example").unwrap();
    tmp_dir.path().to_path_buf()
}

pub fn create_file(file_path: &Path) {
    let prefix = file_path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let _ = File::create(&file_path).expect("failed to create a file");
}

#[derive(Default)]
pub struct TestMetaDataStore {
    metadata: Vec<DocumentMetadata>,
}

impl YggdrasilMetadataStore for TestMetaDataStore {
    fn add_document(&mut self, document_metadata: &DocumentMetadata) {
        self.metadata.push(document_metadata.clone())
    }
}
