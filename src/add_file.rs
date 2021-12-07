use std::{
    fmt::{format, Error},
    path::{Path, PathBuf},
};

use uuid::Uuid;

// Id of a document stored in Yggdrasil
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YggId {
    id: Uuid,
}

impl ToString for YggId {
    fn to_string(&self) -> String {
        self.id.to_hyphenated().to_string()
    }
}

impl YggId {
    pub fn new(id: &Uuid) -> YggId {
        YggId { id: *id }
    }

    pub fn generate() -> YggId {
        YggId { id: Uuid::new_v4() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddFileError;

pub trait YggdrasilMetadataStore {
    fn add_document(&mut self, metadata: &DocumentMetadata);
}

// Stores Yggdrasil metadata in .ygg files.
// Each .ygg file contains storage path to the document
pub struct FileSystemMetadataStore {
    store_root: PathBuf,
}

impl FileSystemMetadataStore {
    pub fn new(store_root: &Path) -> FileSystemMetadataStore {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentMetadata {
    pub id: YggId,
    pub storage_path: String,
}

pub fn add_document(
    yggdrasil_storage_path: &Path,
    metadata_store: &mut dyn YggdrasilMetadataStore,
) -> Result<YggId, AddFileError> {
    if !yggdrasil_storage_path.exists() {
        return Err(AddFileError {});
    }

    let document_id = YggId::generate();
    let metadata = DocumentMetadata {
        id: document_id.clone(),
        storage_path: yggdrasil_storage_path.display().to_string(),
    };

    metadata_store.add_document(&metadata);

    Ok(document_id)
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn exploration() {
    //     let path = "documents/localisation/foo.txt";

    //     assert_eq!(2 + 2, 4);
    // }
}
