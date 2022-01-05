use crate::document::YggId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentMetadata {
    pub id: YggId,
    pub storage_path: String,
}

pub trait YggdrasilMetadataPersister {
    fn save(&mut self, metadata: &DocumentMetadata);
}
