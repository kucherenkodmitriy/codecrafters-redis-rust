use crate::resp_parser::domain::stream_chunking_service::StringCommand;

pub enum RespCommand {
    Ping {
        message: Option<Vec<u8>>,
    },
    //...
}

impl RespCommand {
    pub fn parse(string_command_with_args: StringCommand) -> Result<RespCommand, String> {
        let mut split_command = string_command_with_args
            .as_str()
            .split("\r\n");
        let command = split_command.next().unwrap();

        match command.to_uppercase().as_str() {
            "PING" => Ok(RespCommand::Ping { message: split_command.next().map(|s| s.as_bytes().to_vec()) }),
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
        let command = RespCommand::parse(StringCommand::new("GGET\r\n".to_string()));
        assert!(command.is_err());
    }
}