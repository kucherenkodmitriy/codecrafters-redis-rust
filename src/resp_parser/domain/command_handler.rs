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
    Ok,
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

    pub fn handle_command(&self, command: RespCommand) -> CommandHandlerResult {
        match &command {
            RespCommand::Ping { message: _message } => {
                CommandHandlerResult::new(command, CommandHandlerResultStatus::Ok)
            }
            // Handle other commands here
        }
    }
}