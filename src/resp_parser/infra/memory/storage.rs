use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct StorageEntry {
    pub value: Vec<u8>,
    pub expires_at: Option<u64>,
}

impl StorageEntry {
    pub fn new(value: Vec<u8>, expires_at: Option<u64>) -> Self {
        Self { value, expires_at }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StorageKey(Vec<u8>);

impl From<Vec<u8>> for StorageKey {
    fn from(vec: Vec<u8>) -> Self {
        StorageKey(vec)
    }
}

impl From<StorageKey> for Vec<u8> {
    fn from(key: StorageKey) -> Self {
        key.0
    }
}

pub struct Storage
{
    pub hash_map: Arc<RwLock<HashMap<Vec<u8>, StorageEntry>>>
}

impl Storage {
    fn new() -> Self {
        Storage {
            hash_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, key: StorageKey, entry: StorageEntry) {
        let mut hash_map_lock = self.hash_map.write().await;
        hash_map_lock.insert(key.into(), entry);
    }

    pub async fn get(&self, key: StorageKey) -> Option<StorageEntry> {
        let hash_map_lock = self.hash_map.read().await;
        hash_map_lock.get::<Vec<u8>>(&key.into()).cloned()
    }
}