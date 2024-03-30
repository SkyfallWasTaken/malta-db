use color_eyre::{eyre::Context, Result};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use std::net::TcpListener;

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .wrap_err("setting default subscriber failed")?;

    let listener = TcpListener::bind("127.0.0.1:6379")?;

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
