use super::document_metadata::DocumentMetadata;
use super::metadata_store::YggdrasilMetadataStore;

// Stores Yggdrasil metadata in .ygg files.
// Each .ygg file contains storage path to the document
pub struct FileSystemMetadataStore {
    store_root: std::path::PathBuf,
}

impl FileSystemMetadataStore {
    pub fn new(store_root: &std::path::Path) -> FileSystemMetadataStore {
        FileSystemMetadataStore {
            store_root: store_root.to_path_buf(),
        }
    }
}

impl YggdrasilMetadataStore for FileSystemMetadataStore {
    fn add_document(&mut self, metadata: &DocumentMetadata) {
        let ygg_filename = format!("{}.ygg", metadata.id.to_string());
        let ygg_path = self.store_root.join(ygg_filename);

        let metadata_as_str = metadata.storage_path.to_string();

        std::fs::write(ygg_path, metadata_as_str).unwrap()
    }
}
