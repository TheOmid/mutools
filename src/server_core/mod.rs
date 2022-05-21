use super::rpc_core::*;

use clap::Parser;
use futures::{future, prelude::*};
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    time::Duration,
};
use tarpc::{
    context,
    server::{self, incoming::Incoming, Channel},
    tokio_serde::formats::Json,
};
use tokio::time;


#[derive(Clone)]
pub struct HelloServer(pub SocketAddr);

#[tarpc::server]
impl World for HelloServer {
    async fn hello(self, _: context::Context, name: String) -> String {
        let sleep_time =
            Duration::from_millis(Uniform::new_inclusive(1, 10).sample(&mut thread_rng()));
        time::sleep(sleep_time).await;
        format!("Hello, {name}! You are connected from {}", "idk")
    }
}
