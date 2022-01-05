use super::document_metadata::{DocumentMetadata, YggdrasilMetadataPersister};

// Metadata Store for Yggdrasil. Implement this trait for adding a new store.
pub trait YggdrasilMetadataStore {
    fn add_document(&mut self, metadata: &DocumentMetadata);
}

impl<T: YggdrasilMetadataStore> YggdrasilMetadataPersister for T {
    fn save(&mut self, metadata: &DocumentMetadata) {
        self.add_document(metadata);
    }
}
