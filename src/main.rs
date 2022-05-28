pub mod player;
pub mod sound;
pub mod synthesizer;
pub mod server;

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
    let mutools_server = std::sync::Arc::new(
	std::sync::Mutex::new(
	    server::MutoolsServer::new()
	)
    );
    
    let rpc_server = mutools_rpc::server::MutoolsRpcServer::new(flags.port);
    rpc_server.serve(mutools_server);
}
