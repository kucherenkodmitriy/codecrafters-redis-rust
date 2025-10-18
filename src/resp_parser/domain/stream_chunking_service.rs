pub struct StringCommand(String);

impl StringCommand {
    pub fn new(s: String) -> Self {
        StringCommand(s)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

pub trait StreamChunkingService {
    fn new() -> Self;
    fn next(&mut self, buffer: &[u8]) -> Result<Vec<StringCommand>, String>;
}
