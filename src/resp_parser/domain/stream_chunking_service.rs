#[derive(Debug)]
pub enum StreamChunkingServiceError {
    IncompleteCommand,
    InvalidFormat,
}

impl std::fmt::Display for StreamChunkingServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StreamChunkingServiceError::IncompleteCommand =>
                write!(f, "Incomplete command"),
            StreamChunkingServiceError::InvalidFormat =>
                write!(f, "Invalid format"),
        }
    }
}

impl std::error::Error for StreamChunkingServiceError {}

pub struct StringCommand(String);

impl std::fmt::Display for StringCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StringCommand {
    pub fn new(s: String) -> Self {
        StringCommand(s)
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub trait StreamChunkingService {
    fn new() -> Self;
    fn next(&mut self, buffer: &[u8]) -> Result<Vec<StringCommand>, StreamChunkingServiceError>;
}
