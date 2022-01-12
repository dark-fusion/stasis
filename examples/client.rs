use stasis::logging::initialize_logger;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    initialize_logger().expect("failed to initialize tracing logger");

    let address = std::env::args()
        .skip(1)
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:16000".into());

    let mut stream = TcpStream::connect(&address).await?;

    let (r, mut w) = stream.split();
    let mut server_lines = BufReader::new(r).lines();
    let mut stdin_lines = BufReader::new(io::stdin()).lines();

    loop {
        tokio::select! {
            line = server_lines.next_line() => match line {
                Ok(Some(line)) => {
                    println!("{}", line);
                },
                Ok(None) => {
                    eprintln!("Received empty response from server");
                }
                Err(_) => break,
            },
            line = stdin_lines.next_line() => match line {
                Ok(line) => {
                    let line = line.unwrap();
                    w.write_all(line.as_bytes()).await?;
                    w.write_all(b"\n").await?;
                }
                Err(_) => break,
            }
        }
    }

    Ok(())
}
