use clap::{Parser,Args, Subcommand};

/// adding parser using clap derive
#[derive(Debug,Parser)]

/// will show the author version and the description
#[clap(author, version, about)]
/// this will propagate the clap version to the all commands and the subcommands
#[clap(propagate_version = true)]
pub struct BoreArgs {
    #[clap(subcommand)]
    // creating a subcommand called Command
    pub command: Command,
}

#[derive(Debug, Subcommand)]
// definition of the subcommands
pub enum Command {
    /// Starts a local proxy to the remote server, Requires local_port, to, port
    Local(LocalCommand),

    /// Runs the remote proxy server, Requires min_port
    Server(ServerCommand),
}
#[derive(Debug,Args)]
pub struct LocalCommand{
    /// local_port --> the local port no to listen on
    #[clap(short,long)]
    pub local_port : u16,
    /// to --> the address of the remote server
    #[clap(short,long)]
    pub to : String,
    /// Optional , port --> port on the remote server to be select , if do not specify it by default takes unsigned 2-Byte int 0
    #[clap(short, long, default_value_t = 0)]
    pub port: u16,
}
#[derive(Debug,Args)]
pub struct ServerCommand{
    /// min_port --> Minimum TCP port number to accept To run the remote proxy server.
    #[clap(long, default_value_t = 1024)]
    pub min_port: u16,
}