use futures::{
    future::{self, Ready},
    prelude::*,
};
use tarpc::{
    client, context,
    server::{self, incoming::Incoming, Channel},
};

// Mutools RPC interface
#[tarpc::service]
pub trait World {
    /// Returns a greeting for name.
    async fn hello(name: String) -> String;
}
