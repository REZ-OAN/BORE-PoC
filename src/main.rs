
use bore_cli_poc::args::{BoreArgs,Command};
use bore_cli_poc::client::Client;
use anyhow::Result;
use bore_cli_poc::server::Server;
use  clap::Parser;


#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = BoreArgs::parse();
    match args.command {
        Command::Local(local_command) => {
            let local_port = local_command.local_port;
            let to = local_command.to;
            let port = local_command.port;
            let client = Client::new(local_port, &to, port).await?;
            client.listen().await?;
        }
        Command::Server(server_command) => {
            let min_port = server_command.min_port;
            Server::new(min_port).listen().await?;
        }
    }

    Ok(())
}