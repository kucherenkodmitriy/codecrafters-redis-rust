pub struct QueryRepository {}

impl QueryRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, key: &str) -> Option<String> {
        None
    }
}