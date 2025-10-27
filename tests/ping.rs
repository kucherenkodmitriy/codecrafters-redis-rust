use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};
use std::{thread, time};

/// Helper to start the server as a background process
fn start_server() -> Child {
    Command::new("cargo")
        .arg("run")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to start server")
}

#[test]
fn test_ping() {
    // Start the server
    let mut child = start_server();
    // Wait for the server to start
    thread::sleep(time::Duration::from_millis(500));

    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:6379").expect("failed to connect");

    // Send RESP PING command: *1\r\n$4\r\nPING\r\n
    let ping_command = b"*1\r\n$4\r\nPING\r\n";
    stream.write_all(ping_command).expect("failed to write");

    // Read the response
    let mut buffer = [0; 128];
    let n = stream.read(&mut buffer).expect("failed to read");
    let response = std::str::from_utf8(&buffer[..n]).expect("invalid utf8");

    // The expected response is "+PONG\r\n"
    assert_eq!(response, "+PONG\r\n");

    // Kill the server
    let _ = child.kill();
}

