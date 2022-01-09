use tokio::io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = std::env::args()
        .skip(1)
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:16000".into());

    let mut stream = TcpStream::connect(&address).await?;

    // Split the stream into reader/writer
    let (reader, mut writer) = stream.split();
    let mut lines_from_server = BufReader::new(reader).lines();
    let mut lines_from_stdin = BufReader::new(stdin()).lines();

    loop {
        tokio::select! {
            line = lines_from_server.next_line() => match line {
                Ok(Some(line)) => {
                    println!("{}", line);
                },
                Ok(None) => {
                    eprintln!("Received nothing from client");
                }
                Err(_) => break,
            },
            line = lines_from_stdin.next_line() => match line {
                Ok(line) => {
                    let line = line.unwrap();
                    writer.write_all(line.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
                Err(_) => break,
            }
        }
    }

    Ok(())
}
