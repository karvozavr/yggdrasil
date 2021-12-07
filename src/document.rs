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
