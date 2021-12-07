mod fixtures;
use std::path::Path;

use yggdrasil::{
    add_file::{add_document, AddFileError},
    metadata::FileSystemMetadataStore,
};

#[test]
fn test_document_does_not_exist() {
    // given
    let dir = fixtures::temp_directory();
    let file_path = dir.join(Path::new("foo/bar.txt"));
    let mut metadata_store = fixtures::TestMetaDataStore::default();

    // when
    let result = add_document(&file_path, &mut metadata_store);

    // then
    assert_eq!(result.unwrap_err(), AddFileError {})
}

#[test]
fn test_document_is_added_with_id() {
    // given
    let dir = fixtures::temp_directory();
    let file_path = dir.join(Path::new("foo/bar.txt"));
    fixtures::create_file(&file_path);

    let mut metadata_store = FileSystemMetadataStore::new(&dir);

    // when
    let result = add_document(&file_path, &mut metadata_store);

    // then
    let document_id = result.expect("Failed to add file");
    let ygg_path = dir.join(Path::new(&format!("{}.ygg", document_id.to_string())));
    assert!(ygg_path.exists(), ".ygg file has to be created")
}
