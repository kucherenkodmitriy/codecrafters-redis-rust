use crate::resp_parser::infra::memory::storage::{Storage, StorageEntry};

pub struct CommandRepository {
    storage: Storage,
}

type Milliseconds = u64;
type Seconds = u64;

pub enum TTL {
    Milliseconds(Milliseconds),
    Seconds(Seconds),
    Indefinite,
}

impl TTL {
    fn to_milliseconds(&self) -> Result<Milliseconds, String> {
        match self {
            TTL::Milliseconds(ms) => Ok(*ms),
            TTL::Seconds(s) => Ok(s * 1000),
            TTL::Indefinite => Err("Indefinite TTL cannot be converted to milliseconds".to_string()),
        }
    }

    fn from_milliseconds(ms: Milliseconds) -> Self {
        TTL::Milliseconds(ms)
    }

    fn from_seconds(s: Seconds) -> Self {
        TTL::Seconds(s)
    }
}

impl CommandRepository {
    pub fn new(storage: Storage) -> Self {
        Self {
            storage,
        }
    }

    pub async fn set(&self, key: Vec<u8>, value: Vec<u8>, ttl: TTL) {
        let ttl_storage = ttl.to_milliseconds().ok().map(|duration_ms| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64 + duration_ms
        });
        self.storage.insert(key.into(), StorageEntry::new(value, ttl_storage)).await;
    }
}