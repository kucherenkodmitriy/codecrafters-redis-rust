use crate::resp_parser::domain::command_handler::{CommandHandlerResult, CommandHandlerResultStatus};
use crate::resp_parser::domain::resp_command::RespCommand;
use crate::resp_parser::domain::resp_response::RespResponse;

pub struct ResponseBuilder {}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create(&self, handler_result: CommandHandlerResult) -> Result<RespResponse, String> {
        let command = handler_result.get_resp_command();
        match command {
            RespCommand::Ping { message } => {
                match handler_result.get_status() {
                    CommandHandlerResultStatus::Ok(_) => {
                        Ok(RespResponse::pong())
                    },
                    _ => Err("Mismatched command result for PING".to_string()),
                }
            },
            RespCommand::Echo { message } => {
                match handler_result.get_status() {
                    CommandHandlerResultStatus::Ok(Some(message)) => {
                        Ok(RespResponse::echo(message))
                    },
                    _ => Err("Mismatched command result for PING".to_string()),
                }
            }
        }
    }
}