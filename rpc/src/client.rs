use super::interface::*;

use tarpc::client::{Channel, Config};
use tokio_serde;

pub struct MutoolsRpcClient {
    client: WorldClient
}

impl MutoolsRpcClient {

    pub async fn new(port: u16) -> Result<Self, ()> {
	let server_addr = (std::net::IpAddr::V6(std::net::Ipv6Addr::LOCALHOST), port);
	let transport = tarpc::serde_transport::tcp::connect(server_addr, tokio_serde::formats::Json::default).await;
	match transport {
	    Ok(t) => Ok(Self {
		client: WorldClient::new(Config::default(), t).spawn(),
	    }),
	    _ => Err(()),
	}
    }

    pub fn get_client(&self) -> &WorldClient {
	&self.client
    }
}

