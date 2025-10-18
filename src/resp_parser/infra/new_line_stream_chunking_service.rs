use crate::resp_parser::domain::stream_chunking_service::StreamChunkingService;
use crate::resp_parser::domain::stream_chunking_service::StringCommand;

pub struct NewLineStreamChunkingService {
    buffer: Vec<u8>,
}

impl StreamChunkingService for NewLineStreamChunkingService {
    fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }
    fn next(&mut self, buffer: &[u8]) -> Result<Vec<StringCommand>, String> {
        let has_trailing_newline = buffer.last() == Some(&b'\n');

        let mut chunks: Vec<Vec<u8>> = buffer
            .split(|&c| c == b'\n')
            .map(|s| s.to_vec())
            .collect();

        if has_trailing_newline {
            chunks.pop();
        }

        if !self.buffer.is_empty() {
            if let Some(first_chunk) = chunks.first_mut() {
                let mut combined = self.buffer.clone();
                combined.append(first_chunk);
                *first_chunk = combined;
                self.buffer.clear();
            }
        }


        if !has_trailing_newline {
            if let Some(last_chunk) = chunks.pop() {
                self.buffer = last_chunk;
            }
        }

        let commands: Vec<StringCommand> = chunks
            .into_iter()
            .filter(|c| !c.is_empty())
            .map(|c| StringCommand::new(String::from_utf8_lossy(&c).to_string()))
            .collect();

        if commands.is_empty() {
            Err("Incomplete command".to_string())
        } else {
            Ok(commands)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_complete_command() {
        let mut service = NewLineStreamChunkingService::new();
        let result = service.next(b"SET key value\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_multiple_complete_commands() {
        let mut service = NewLineStreamChunkingService::new();
        let result = service.next(b"SET key value\nGET key\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_incomplete_command() {
        let mut service = NewLineStreamChunkingService::new();
        let result = service.next(b"SET key value");
        assert!(result.is_err());
    }

    #[test]
    fn test_incomplete_command_and_then_completed_with_next_call() {
        let mut service = NewLineStreamChunkingService::new();
        let result = service.next(b"SET key va");
        assert!(result.is_err());
        let result = service.next(b"lue\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_incomplete_command_and_then_completed_with_next_call_and_buffer() {
        let mut service = NewLineStreamChunkingService::new();
        let result = service.next(b"SET key va");
        assert!(result.is_err());
        let result = service.next(b"lue\nGET key\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_incomplete_command_and_then_completed_with_next_call_new_line() {
        let mut service = NewLineStreamChunkingService::new();
        let result = service.next(b"SET key value");
        assert!(result.is_err());
        let result = service.next(b"\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_incomplete_command_and_then_incomplete_again_and_then_completed() {
        let mut service = NewLineStreamChunkingService::new();
        let result = service.next(b"SET key va");
        assert!(result.is_err());
        let result = service.next(b"l");
        assert!(result.is_err());
        let result = service.next(b"ue\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_with_success_and_then_new_command_with_success() {
        let mut service = NewLineStreamChunkingService::new();
        let result = service.next(b"SET key value\nGET key\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
        let result = service.next(b"SET key value\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_command_without_new_line_and_then_several_new_lines() {
        let mut service = NewLineStreamChunkingService::new();
        let result = service.next(b"SET key value");
        assert!(result.is_err());
        let result = service.next(b"\n\n\n\n\n\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_command_without_new_line_and_then_several_new_lines_and_then_command() {
        let mut service = NewLineStreamChunkingService::new();
        let result = service.next(b"SET key value");
        assert!(result.is_err());
        let result = service.next(b"\n\n\n\n\n\nGET key\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_several_new_lines_then_command_and_new_lines() {
        let mut service = NewLineStreamChunkingService::new();
        let result = service.next(b"\n\n\n\n\n\nGET key\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }
}