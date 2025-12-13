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
            RespCommand::Ping { message: _ } => {
                match handler_result.get_status() {
                    CommandHandlerResultStatus::Ok(_) => Ok(RespResponse::pong()),
                }
            },
            RespCommand::Echo { message: _ } => {
                match handler_result.get_status() {
                    CommandHandlerResultStatus::Ok(Some(msg)) => Ok(RespResponse::echo(msg)),
                    _ => Err("Mismatched command result for ECHO".to_string()),
                }
            },
            RespCommand::Set { key: _, value: _ } => {
                match handler_result.get_status() {
                    CommandHandlerResultStatus::Ok(_) => Ok(RespResponse::set()),
                }
            },
            RespCommand::Get { key : _ } => {
                match handler_result.get_status() {
                    CommandHandlerResultStatus::Ok(Some(value)) => Ok(RespResponse::get(value)),
                    CommandHandlerResultStatus::Ok(None) => Ok(RespResponse::null()),
                }
            },
        }
    }
}