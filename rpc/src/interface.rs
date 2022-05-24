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

    // Misc API
    async fn get_version() -> String;

    // Project API
    async fn get_num_project_descriptors() -> usize; 

    // Transform API


    // Playback API
    

}
