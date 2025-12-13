use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type Storage = Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>;