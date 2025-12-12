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

    pub fn get(&self, key: &str) -> Option<String> {
        None
    }
}