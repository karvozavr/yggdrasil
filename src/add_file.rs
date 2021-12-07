use crate::metadata::DocumentMetadata;
use crate::{document::YggId, metadata::YggdrasilMetadataPersister};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddFileError;

pub fn add_document(
    yggdrasil_storage_path: &Path,
    metadata_store: &mut dyn YggdrasilMetadataPersister,
) -> Result<YggId, AddFileError> {
    if !yggdrasil_storage_path.exists() {
        return Err(AddFileError {});
    }

    let document_id = YggId::generate();
    let metadata = DocumentMetadata {
        id: document_id.clone(),
        storage_path: yggdrasil_storage_path.display().to_string(),
    };

    metadata_store.save(&metadata);

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
