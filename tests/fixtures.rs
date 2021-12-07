use std::path::PathBuf;

use tempdir::TempDir;
use yggdrasil::metadata::{DocumentMetadata, YggdrasilMetadataStore};

pub fn temp_directory() -> PathBuf {
    let tmp_dir = TempDir::new("example").unwrap();
    tmp_dir.path().to_path_buf()
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
