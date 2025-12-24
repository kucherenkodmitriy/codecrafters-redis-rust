use crate::resp_parser::domain::stream_chunking_service::StringCommand;

pub enum RespCommand {
    Ping {
        message: Option<Vec<u8>>,
    },
    Echo {
        message: Option<Vec<u8>>,
    },
    Set {
        key: Vec<u8>,
        value: Vec<u8>,
        ttl: Option<RespTtl>,
    },
    Get {
        key: Vec<u8>,
    },
    //...
}

pub enum RespTtl {
    Seconds(u64),
    Milliseconds(u64),
}

impl RespTtl {
    pub fn from_seconds(seconds: u64) -> Self {
        RespTtl::Seconds(seconds)
    }

    pub fn from_milliseconds(milliseconds: u64) -> Self {
        RespTtl::Milliseconds(milliseconds)
    }

    pub fn to_seconds(&self) -> u64 {
        match self {
            RespTtl::Seconds(s) => *s,
            RespTtl::Milliseconds(ms) => *ms / 1000,
        }
    }
}

impl RespCommand {
    pub fn parse(string_command_with_args: StringCommand) -> Result<RespCommand, String> {
        let mut split_command = string_command_with_args
            .as_str()
            .split("\r\n");
        let command = split_command.next().unwrap();

        match command.to_uppercase().as_str() {
            "PING" => Ok(RespCommand::Ping { message: split_command.next().map(|s| s.as_bytes().to_vec()) }),
            "ECHO" => Ok(RespCommand::Echo { message: split_command.next().map(|s| s.as_bytes().to_vec()) }),
            "SET" => Ok(RespCommand::Set {
                key: split_command.next().unwrap().as_bytes().to_vec(),
                value: split_command.next().unwrap().as_bytes().to_vec(),
                ttl: None,
            }),
            "GET" => Ok(RespCommand::Get {
                key: split_command.next().unwrap().as_bytes().to_vec(),
            }),
            _ => Err(format!("Unknown command: {}", command)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        let command = RespCommand::parse(StringCommand::new("PING\r\n".to_string()));
        assert!(command.is_ok());
    }

    #[test]
    fn test_parse_command_unknown_command() {
        let command = RespCommand::parse(StringCommand::new("GET\r\n".to_string()));
        assert!(command.is_err());
    }

    #[test]
    fn test_echo_command() {
        let command = RespCommand::parse(StringCommand::new("ECHO\r\nHey\r\n".to_string()));
        assert!(command.is_ok());
        let command = command.unwrap();
        match command {
            RespCommand::Echo { message } => {
                assert_eq!(message, Some("Hey".as_bytes().to_vec()));
            },
            _ => panic!("Unexpected command type")
        }
    }

    #[test]
    fn test_set_command() {
        let command = RespCommand::parse(StringCommand::new("SET\r\nkey\r\nvalue\r\n".to_string()));
        assert!(command.is_ok());
        let command = command.unwrap();
        match command {
            RespCommand::Set { key, value, ttl } => {
                assert_eq!(key, "key".as_bytes().to_vec());
                assert_eq!(value, "value".as_bytes().to_vec());
            },
            _ => panic!("Unexpected command type")
        }   
    }

    #[test]
    fn test_get_command() {
        let command = RespCommand::parse(StringCommand::new("GET\r\nkey\r\n".to_string()));
        assert!(command.is_ok());
        let command = command.unwrap();
        match command {
            RespCommand::Get { key } => {
                assert_eq!(key, "key".as_bytes().to_vec());
            },
            _ => panic!("Unexpected command type")
        }
    }
}