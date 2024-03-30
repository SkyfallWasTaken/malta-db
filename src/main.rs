use color_eyre::{eyre::Context, Result};
use tracing::{debug, error, info, Level};
use tracing_subscriber::FmtSubscriber;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const ADDR: &str = "127.0.0.1:6379";

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .wrap_err("setting default subscriber failed")?;

    let listener = TcpListener::bind(ADDR)?;
    info!("Started listening on {ADDR}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                debug!("accepted new connection");
                handle_client(stream)?
            }
            Err(e) => {
                error!("{e}");
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let s = std::str::from_utf8(&buf[..bytes_read])?;
        info!("{s}");

        stream.write_all(b"+PONG\r\n")?;
    }
}
