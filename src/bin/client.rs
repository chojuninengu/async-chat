use async_chat::{FromServer, utils};
use async_std::{io::BufReader, net, prelude::FutureExt, stream::StreamExt, task};
fn main() -> anyhow::Result<()> {
    let address = std::env::args().nth(1).expect("Usage: client ADDRESS:PORT");

    task::block_on(async {
        let socket = net::TcpStream::connect(address).await?;
        socket.set_nodelay(true)?;
        let to_server = send_commands(socket.clone());
        let from_server = handle_replies(socket);
        // waits for client to finish before server starts handling replies
        from_server.race(to_server).await?;
        Ok(())
    })
}

async fn send_commands(mut to_server: net::TcpStream) -> anyhow::Result<()> {
    // Todo: Implement use clap to parse command line arguments and print help message

    // send_as_json(&mut to_server, &result).await?;

    todo!()
}

async fn handle_replies(from_server: net::TcpStream) -> anyhow::Result<()> {
    let buffered = BufReader::new(from_server);
    let mut reply_stream = utils::receive_as_json(buffered);
    while let Some(reply) = reply_stream.next().await {
        let reply = reply?;
        match reply {
            FromServer::Message {
                group_name,
                message,
            } => {
                println!("message posted to {}: {}", group_name, message);
            }
            FromServer::Error(error) => {
                eprintln!("Error: {}", error);
            }
        }
    }

    Ok(())
}
