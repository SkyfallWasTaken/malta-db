use color_eyre::{eyre::Context, Result};
use tracing::{debug, error, info, Level};
use tracing_subscriber::FmtSubscriber;

use std::net::TcpListener;

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
            Ok(_stream) => {
                debug!("accepted new connection");
            }
            Err(e) => {
                error!("{e}");
            }
        }
    }

    Ok(())
}
