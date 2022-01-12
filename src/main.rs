use std::sync::Arc;

use futures::SinkExt;
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use tracing::{error, info};

use stasis::database::{handle_command, Database, MAX_PAYLOAD_LENGTH};
use stasis::logging::initialize_logger;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    initialize_logger().expect("failed to initialize tracing logger");

    let address = std::env::args()
        .skip(1)
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:16000".into());

    let listener = TcpListener::bind(&address).await?;
    info!("Server started. Listening at: {}", listener.local_addr()?);

    let database = Arc::new(Database::new());
    let initial_state = ("foo", "bar".to_string());
    database.set(initial_state.0, &initial_state.1);

    loop {
        match listener.accept().await {
            Ok((stream, socket_addr)) => {
                let database = database.clone();

                info!("New connection from: {:?}", socket_addr);

                tokio::spawn(async move {
                    // N.B. Current implementation uses a lines-based codec.
                    // `LinesCodec` decodes incoming byte streams to lines and
                    // encodes outgoing data back into streams of bytes.
                    let mut lines =
                        Framed::new(stream, LinesCodec::new_with_max_length(MAX_PAYLOAD_LENGTH));

                    while let Some(result) = lines.next().await {
                        match result {
                            Ok(cmd) => {
                                let response = handle_command(&cmd, &database);
                                let serialized = response.to_bytes();

                                if let Err(err) = lines.send(serialized.as_str()).await {
                                    error!("failed to send response; {:?}", err);
                                }
                            }
                            Err(err) => {
                                error!("failed to decode data from stream; {:?}", err);
                            }
                        }
                    }
                });
            }
            Err(err) => error!("failed to accept socket from {:?}", err),
        }
    }
}
