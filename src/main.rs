pub mod player;
pub mod sound;
pub mod synthesizer;

use clap::Parser;

#[derive(Parser)]
struct Flags {
    /// Sets the port number to listen on.
    #[clap(long)]
    port: u16,
}

fn main() -> () {
    let flags = Flags::parse();
    println!("Mutools server initializing...");

    let rpc_server = mutools_rpc::server::MutoolsRPCServer::new(flags.port);
    rpc_server.serve();
    
}
