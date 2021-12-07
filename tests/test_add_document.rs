mod fixtures;
use fixtures::temp_directory;
use std::fs::File;
use std::path::Path;

use yggdrasil::add_file::{
    add_document, AddFileError, DocumentMetadata, FileSystemMetadataStore, YggdrasilMetadataStore,
};

struct TestMetaDataStore {
    metadata: Vec<DocumentMetadata>,
}

impl TestMetaDataStore {
    fn new() -> TestMetaDataStore {
        TestMetaDataStore {
            metadata: Vec::new(),
        }
    }
}

impl YggdrasilMetadataStore for TestMetaDataStore {
    fn add_document(&mut self, document_metadata: &DocumentMetadata) {
        self.metadata.push(document_metadata.clone())
    }
}

#[test]
fn test_document_does_not_exist() {
    // given
    let dir = temp_directory();
    let file_path = dir.join(Path::new("foo/bar.txt"));
    let mut metadata_store = TestMetaDataStore::new();

    // when
    let result = add_document(&file_path, &mut metadata_store);

    // then
    assert_eq!(result.unwrap_err(), AddFileError {})
}

#[test]
fn test_document_is_added_with_id() {
    // given
    let dir = temp_directory();
    let file_path = dir.join(Path::new("foo/bar.txt"));
    let prefix = file_path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let _ = File::create(&file_path).expect("failed to create a file");

    let mut metadata_store = FileSystemMetadataStore::new(&dir);

    // when
    let result = add_document(&file_path, &mut metadata_store);

    // then
    let document_id = result.expect("Failed to add file");
    let ygg_path = dir.join(Path::new(&format!("{}.ygg", document_id.to_string())));
    assert!(ygg_path.exists(), ".ygg file has to be created")
}
