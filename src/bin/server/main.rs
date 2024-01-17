use futures::StreamExt;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use simple_redis::parser;
use simple_redis::config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Listening on: {}:{}", config::ADDRESS, config::PORT);
    let addr = format!("{}:{}", config::ADDRESS, config::PORT);
    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let mut framed_stream = Framed::new(socket, LengthDelimitedCodec::new());
        tokio::spawn(async move {
           while let Some(msg) = framed_stream.next().await {
            match msg {
                Ok(msg) => {
                    let command = String::from_utf8(msg.to_vec())
                                .expect("error while converting to string commaind");
                    println!("command: {command}");
                    let cmd = parser::parse(&command);
                    println!("parsed cmd={:?}", cmd);
                }
                Err(e) => {
                    println!("{e:?}");
                }
            }
           }

        });
    }
}