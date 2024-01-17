use futures::{SinkExt, StreamExt};
use std::io;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use simple_redis::config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = format!("{}:{}", config::ADDRESS, config::PORT);
	let stream = TcpStream::connect(&addr).await?;
	let mut framed_stream = Framed::new(stream, LengthDelimitedCodec::new());
	let mut buffer = String::new();
	loop {
        io::stdin().read_line(&mut buffer)?;
        println!("Get {} from stdin", buffer);
        framed_stream.send(buffer.clone().into()).await?;
		
		if let Some(msg) = framed_stream.next().await {
			match msg {
				Ok(msg) => {
					let res = String::from_utf8(msg.to_vec())?;
					println!("{}", res);
				}
				Err(e) => return Err(e.into()),
			}
		}
        buffer.clear();
    }

}