use smoke_test::{telemetry::{get_subscriber, init_subscriber}, configuration::get_config};
use tokio::{net::{TcpListener, TcpStream}, io::{AsyncWriteExt, AsyncReadExt}};
use tracing::{event, Level};
use std::error::Error;
use anyhow::Result;

#[tracing::instrument(
    name = "handle_stream"
)]
async fn handle(mut stream: TcpStream) -> Result<()> {

    loop { 
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break;
        }

        stream.write_all(&buf[0..n]).await?;
    }

    stream.shutdown().await?;

    Ok(())
}
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    let subscriber = get_subscriber("echo".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    

    let config = get_config().expect("Failed to read configuration");
    let addr = format!("{}:{}",config.application.host, config.application.port);
    let listener = TcpListener::bind(&addr).await?;
    event!(Level::INFO, "Listening on {}", &addr);
    
    loop {

        let (stream, _) = listener.accept().await?;
        handle(stream).await?;
    }   
}

