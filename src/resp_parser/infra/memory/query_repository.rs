use crate::resp_parser::infra::memory::storage::Storage;

pub struct QueryRepository {
    storage: Storage,
}

impl QueryRepository {
    pub fn new(storage: Storage) -> Self {
        Self {
            storage,
        }
    }

    pub async fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        self.storage.get(key.into()).await.map(|entry| entry.value)
    }
}