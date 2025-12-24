use crate::resp_parser::domain::stream_chunking_service::{StreamChunkingService, StreamChunkingServiceError};
use crate::resp_parser::domain::stream_chunking_service::StringCommand;

pub struct NewLineStreamChunkingService {
    buffer: Vec<u8>,
    state_stack: Vec<ParseState>,
    current_parts: Vec<String>,
}

#[derive(Debug, Clone)]
enum ParseState {
    ArrayCount,
    BulkStringLength,
    BulkStringData(usize),
}

impl StreamChunkingService for NewLineStreamChunkingService {
    fn new() -> Self {
        Self {
            buffer: Vec::new(),
            state_stack: vec![ParseState::ArrayCount],
            current_parts: Vec::new(),
        }
    }
    fn next(&mut self, buffer: &[u8]) -> Result<Vec<StringCommand>, StreamChunkingServiceError> {
        self.buffer.extend_from_slice(buffer);
        let mut commands = Vec::new();

        loop {
            match self.parse_next() {
                Ok(Some(command)) => commands.push(command),
                Ok(None) => break,
                Err(e) => {
                    if commands.is_empty() {
                        return Err(e);
                    }
                    break;
                }
            }
        }

        if commands.is_empty() {
            Err(StreamChunkingServiceError::IncompleteCommand)
        } else {
            Ok(commands)
        }
    }
}

impl NewLineStreamChunkingService {
    fn parse_next(&mut self) -> Result<Option<StringCommand>, StreamChunkingServiceError> {
        while let Some(state) = self.state_stack.last().cloned() {
            match state {
                ParseState::ArrayCount => {
                    let count = match self.read_integer(b'*')? {
                        Some(n) => n,
                        None => return Ok(None),
                    };

                    self.state_stack.pop();

                    for _ in 0..count {
                        self.state_stack.push(ParseState::BulkStringLength);
                    }
                }

                ParseState::BulkStringLength => {
                    let length = match self.read_integer(b'$')? {
                        Some(n) => n,
                        None => return Ok(None),
                    };

                    self.state_stack.pop();
                    self.state_stack.push(ParseState::BulkStringData(length));
                }
                ParseState::BulkStringData(length) => {
                    let data = match self.read_bulk_string(length)? {
                        Some(s) => s,
                        None => return Ok(None),
                    };

                    self.current_parts.push(data);
                    self.state_stack.pop();
                }
            }
        }

        if self.current_parts.is_empty() {
            return Ok(None);
        }

        let command = StringCommand::new(
            std::mem::take(&mut self.current_parts)
                .join("\r\n") + "\r\n"
        );

        // Reset for next command
        self.state_stack.push(ParseState::ArrayCount);

        Ok(Some(command))
    }

    fn read_integer(&mut self, prefix: u8) -> Result<Option<usize>, StreamChunkingServiceError> {
        if self.buffer.is_empty() {
            return Ok(None);
        }

        if self.buffer[0] != prefix {
            return Err(StreamChunkingServiceError::InvalidFormat);
        }

        let crlf_pos = match self.find_crlf() {
            Some(pos) => pos,
            None => return Ok(None),
        };

        let num_str = std::str::from_utf8(&self.buffer[1..crlf_pos])
            .map_err(|_| StreamChunkingServiceError::InvalidFormat)?;

        let num = num_str.parse::<usize>()
            .map_err(|_| StreamChunkingServiceError::InvalidFormat)?;

        self.buffer.drain(..crlf_pos + 2);
        Ok(Some(num))
    }

    fn read_bulk_string(&mut self, length: usize) -> Result<Option<String>, StreamChunkingServiceError> {
        if self.buffer.len() < length + 2 {
            return Ok(None);
        }

        if self.buffer[length] != b'\r' || self.buffer[length + 1] != b'\n' {
            return Err(StreamChunkingServiceError::InvalidFormat);
        }

        let bytes = self.buffer.drain(..length).collect::<Vec<_>>();
        self.buffer.drain(..2);

        String::from_utf8(bytes)
            .map(Some)
            .map_err(|_| StreamChunkingServiceError::InvalidFormat)
    }

    fn find_crlf(&self) -> Option<usize> {
        self.buffer.windows(2).position(|w| w == b"\r\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_line_stream_chunking_service() {
        let mut service = NewLineStreamChunkingService::new();
        let input = b"*2\r\n$3\r\nGET\r\n$3\r\nkey\r\n";
        let command = service.next(input).unwrap();
        assert_eq!(command.len(), 1);
        assert_eq!(command[0].to_string(), "GET\r\nkey\r\n");
    }

    #[test]
    fn test_new_line_stream_chunking_service_invalid_format() {
        let mut service = NewLineStreamChunkingService::new();
        let input = b"*2\r\n$3\r\nGET\r\n$4\r\nkey\r\n";
        let result = service.next(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_line_stream_chunking_service_empty_buffer() {
        let mut service = NewLineStreamChunkingService::new();
        let input = b"";
        let result = service.next(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_line_stream_chunking_service_half_command_and_then_complete() {
        let mut service = NewLineStreamChunkingService::new();
        let input = b"*2\r\n$3\r\nGET\r\n$3\r\nkey\r\n*3\r\n$3\r\nSET\r\n";
        let command = service.next(input).unwrap();
        assert_eq!(command.len(), 1);
        assert_eq!(command[0].to_string(), "GET\r\nkey\r\n");
        // adding the rest of the command
        let input = b"$3\r\nkey\r\n$1\r\nS\r\n";
        let command = service.next(input).unwrap();
        assert_eq!(command.len(), 1);
        assert_eq!(command[0].to_string(), "SET\r\nkey\r\nS\r\n");
    }

    #[test]
    fn test_new_line_stream_chunking_service_ping() {
        let mut service = NewLineStreamChunkingService::new();
        let input = b"*1\r\n$4\r\nPING\r\n";
        let command = service.next(input).unwrap();
        assert_eq!(command.len(), 1);
        assert_eq!(command[0].to_string(), "PING\r\n");
    }

    #[test]
    fn test_new_line_stream_chunking_service_set_with_ttl() {
        let mut service = NewLineStreamChunkingService::new();
        let input = b"*4\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n$2\r\n10\r\n";
        let command = service.next(input).unwrap();
        assert_eq!(command.len(), 1);
        assert_eq!(command[0].to_string(), "SET\r\nkey\r\nvalue\r\n10\r\n");
    }

    #[test]
    fn test_new_line_stream_chunking_service_set_with_ttl_milliseconds() {
        // SET mykey value PX 1000
        let mut service = NewLineStreamChunkingService::new();
        let input = b"*5\r\n$3\r\nSET\r\n$5\r\nmykey\r\n$5\r\nvalue\r\n$2\r\nPX\r\n$4\r\n1000\r\n";
        let command = service.next(input).unwrap();
        assert_eq!(command.len(), 1);
        assert_eq!(command[0].to_string(), "SET\r\nmykey\r\nvalue\r\nPX\r\n1000\r\n");
    }
}