use crate::resp_parser::domain::resp_command::RespCommand;
use crate::resp_parser::infra::memory::command_repository::CommandRepository;
use crate::resp_parser::infra::memory::query_repository::QueryRepository;

pub struct CommandHandler {
    command_repository: CommandRepository,
    query_repository: QueryRepository,
}

pub enum CommandHandlerError {
    UnknownCommand,
    InvalidArguments,
    ExecutionError(String),
    RepositoryError(String),
}

pub enum CommandHandlerResultStatus {
    Ok(Option<String>),
    Err(CommandHandlerError),
}

pub struct CommandHandlerResult {
    resp_command: RespCommand,
    status: CommandHandlerResultStatus,
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

    pub async fn handle_command(&self, command: RespCommand) -> CommandHandlerResult {
        match &command {
            RespCommand::Ping { message } => {
                CommandHandlerResult::new(command, CommandHandlerResultStatus::Ok(None))
            },
            RespCommand::Echo { message } => {
                let message = message
                    .as_ref()
                    .map(|bytes| String::from_utf8_lossy(&bytes).to_string());
                CommandHandlerResult::new(command, CommandHandlerResultStatus::Ok(message))
            },
            RespCommand::Set { key, value } => {
                self.command_repository.set(key.clone(), value.clone()).await;
                CommandHandlerResult::new(command, CommandHandlerResultStatus::Ok(None))
            }
            // Handle other commands here
        }
    }
}