use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Storage = Arc<Mutex<HashMap<Vec<u8>, Vec<u8>>>>;