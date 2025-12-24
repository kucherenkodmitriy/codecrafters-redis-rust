use crate::resp_parser::domain::resp_command::{RespCommand, RespTtl};
use crate::resp_parser::infra::memory::command_repository::{CommandRepository, TTL};
use crate::resp_parser::infra::memory::query_repository::QueryRepository;

pub struct CommandHandler {
    command_repository: CommandRepository,
    query_repository: QueryRepository,
}

pub enum CommandHandlerResultStatus {
    Ok(Option<Vec<u8>>),
}

pub struct CommandHandlerResult {
    resp_command: RespCommand,
    status: CommandHandlerResultStatus,
}

impl From<&RespTtl> for TTL {
    fn from(resp_ttl: &RespTtl) -> Self {
        match resp_ttl {
            RespTtl::Seconds(s) => TTL::Seconds(s.clone()),
            RespTtl::Milliseconds(ms) => TTL::Milliseconds(ms.clone()),
        }
    }
}

impl CommandHandlerResult {
    pub fn new(resp_command: RespCommand, status: CommandHandlerResultStatus) -> Self {
        Self {
            resp_command,
            status,
        }
    }

    pub fn get_status(&self) -> &CommandHandlerResultStatus {
        &self.status
    }

    pub fn get_resp_command(&self) -> &RespCommand {
        &self.resp_command
    }
}

impl CommandHandler {
    pub fn new(command_repository: CommandRepository, query_repository: QueryRepository) -> Self {
        CommandHandler {
            command_repository,
            query_repository,
        }
    }

    // probably better to pass ownership of command here and return ownership in result
    pub async fn handle_command(&self, command: RespCommand) -> CommandHandlerResult {
        match &command {
            RespCommand::Ping { message: _ } => {
                CommandHandlerResult::new(command, CommandHandlerResultStatus::Ok(None))
            },
            RespCommand::Echo { message } => {
                CommandHandlerResult::new(command, CommandHandlerResultStatus::Ok(message.clone()))
            },
            RespCommand::Set { key, value, ttl } => {
                let ttl: TTL = match ttl {
                    Some(resp_ttl) => TTL::from(resp_ttl),
                    None => TTL::Indefinite,
                };
                self.command_repository.set(key.clone(), value.clone(), ttl).await;
                CommandHandlerResult::new(command, CommandHandlerResultStatus::Ok(None))
            },
            RespCommand::Get { key } => {
                match self.query_repository.get(key.clone()).await {
                    Some(value) => {
                        CommandHandlerResult::new(command, CommandHandlerResultStatus::Ok(Some(value)))
                    },
                    None => {
                        CommandHandlerResult::new(command, CommandHandlerResultStatus::Ok(None))
                    }
                }
            },
            // Handle other commands here
        }
    }
}