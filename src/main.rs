mod cli;
mod config;

use log::{debug, error, info};
use simplelog::{TermLogger, TerminalMode};
use std::fs;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::config::Config;

const DEFAULT_PORT: u16 = 8888;

fn recv(ring: &rio::Rio, stream: &TcpStream, buf: &mut [u8]) -> io::Result<usize> {
    let bytes = ring.recv(stream, &buf).wait()?;
    return Ok(bytes);
}

fn resp(ring: &rio::Rio, stream: &TcpStream) -> io::Result<()> {
    let contents = fs::read_to_string("test.html")?;
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    info!("Sending response");
    ring.send(stream, &response.as_bytes()).wait()?;
    Ok(())
}

fn handle_request(ring: rio::Rio, stream: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 4096];
    let bytes = recv(&ring, &stream, &mut buf)?;
    debug!(
        "Received {} bytes: {}",
        bytes,
        String::from_utf8_lossy(&buf)
    );

    if let Err(e) = resp(&ring, &stream) {
        error!("{}", e);
    }
    info!("Connection closed");
    Ok(())
}

fn main() -> io::Result<()> {
    let config = Config::from_args();
    TermLogger::init(
        config.log_level(),
        simplelog::Config::default(),
        TerminalMode::Mixed,
    )
    .expect("Failed to initiate logger");
    let io_uring = rio::new()?;
    let acceptor = TcpListener::bind(config.socket_addr())?;
    info!("Listening on {}:{}", config.ip(), config.port());
    loop {
        let stream = io_uring.accept(&acceptor).wait()?;
        info!("Incomming connection from: {}", stream.peer_addr()?);
        let ring = io_uring.clone();
        let _child = thread::spawn(move || {
            if let Err(e) = handle_request(ring, stream) {
                error!("{}", e);
            }
        });
    }
}
