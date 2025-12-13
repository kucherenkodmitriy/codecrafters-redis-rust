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

    pub async fn get(&self, key: Vec<u8>) -> Option<String> {
        let storage_lock = self.storage.read().await;

        storage_lock.get::<Vec<u8>>(key.as_ref()).map(|value| {
            String::from_utf8_lossy(value).to_string()
        })
    }
}