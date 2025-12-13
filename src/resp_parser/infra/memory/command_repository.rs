use crate::resp_parser::infra::memory::storage::Storage;

pub struct CommandRepository {
    storage: Storage,
}

impl CommandRepository {
    pub fn new(storage: Storage) -> Self {
        Self {
            storage,
        }
    }

    pub async fn set(&self, key: Vec<u8>, value: Vec<u8>) {
        let mut storage_lock = self.storage.write().await;
        storage_lock.insert(key, value);
    }
}