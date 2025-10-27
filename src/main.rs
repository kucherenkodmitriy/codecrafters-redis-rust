use std::io::{Read, Write};
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use crate::resp_parser::domain::command_handler::CommandHandler;
use crate::resp_parser::domain::stream_chunking_service::{StreamChunkingService, StreamChunkingServiceError};
use crate::resp_parser::infra::new_line_stream_chunking_service::NewLineStreamChunkingService;
use crate::resp_parser::domain::resp_command::RespCommand;
use crate::resp_parser::infra::memory::command_repository::CommandRepository;
use crate::resp_parser::infra::memory::query_repository::QueryRepository;

mod resp_parser;

#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream) {
    let mut buffer = [0; 512];
    let mut chunking_service = NewLineStreamChunkingService::new();

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                println!("Connection closed");
                return;
            }
            Ok(bytes_read) => {
                println!("Received {} bytes", &buffer[..bytes_read].len());
                match chunking_service.next(&buffer[..bytes_read]) {
                    Ok(commands) => {
                        if !commands.is_empty() {
                            println!("Received {} commands", commands.len());
                            for cmd in commands {
                                println!("{}", cmd);
                                let command = RespCommand::parse(cmd);
                                match command {
                                    Ok(resp_command) => {
                                        match process_command(resp_command) {
                                            Ok(response) => {
                                                write_response(&mut stream, &response).await;
                                            },
                                            Err(e) => {
                                                let error_response = format!("-ERR {}\r\n", e);
                                                write_response(&mut stream, &error_response);
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        let error_response = format!("-ERR {}\r\n", e);
                                        write_response(&mut stream, &error_response);
                                    }
                                }
                            }
                        }
                    },
                    Err(StreamChunkingServiceError::IncompleteCommand) => {
                        continue;
                    },
                    Err(StreamChunkingServiceError::InvalidFormat) => {
                        println!("Invalid format");
                        return;
                    }
                }
            }
            Err(e) => {
                println!("Failed to read from connection: {}", e);
            }
        }
    }
}

fn process_command(command: RespCommand) -> Result<String, String> {
    let handler = CommandHandler::new(
        CommandRepository::new(),
        QueryRepository::new(),
    );
    let handler_result = handler.handle_command(command);
    let response_factory = resp_parser::domain::response_builder::ResponseBuilder::new();
    response_factory.create(handler_result)
        .map(|resp_response| resp_response.to_resp())
}

async fn write_response(stream: &mut tokio::net::TcpStream, response: &str) {
    use tokio::io::AsyncWriteExt;
    stream.write_all(response.as_bytes()).await.unwrap();
}