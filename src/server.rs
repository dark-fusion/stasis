use std::io;
use std::net::SocketAddr;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

pub async fn start() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let address = args.next().unwrap_or_else(|| "0.0.0.0:15550".into());
    let address = address
        .parse::<SocketAddr>()
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err.to_string()))?;

    let listener = TcpListener::bind(&address).await?;

    serve(listener).await?;

    Ok(())
}

pub async fn serve(listener: TcpListener) -> std::io::Result<()> {
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("New connection received.");
                handle_connection(stream).await?;
            }
            Err(err) => {
                eprintln!("Error accepting socket: {err:?}")
            }
        }
    }
}

pub async fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];

    let n = stream.read(&mut buf).await?;
    println!("Read {n} bytes from stream");
    println!("{buf:#?}");

    Ok(())
}
