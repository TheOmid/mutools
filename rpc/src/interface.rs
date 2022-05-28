use futures::{
    future::{self, Ready},
    prelude::*,
};
use tarpc::{
    client, context,
    server::{self, incoming::Incoming, Channel},
};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectDescriptor {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoundDescriptor {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BufferDescriptor {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionDescriptor {
    id: i32
}

impl TransactionDescriptor {
    pub fn new() -> Self {
	Self {
	    id: 123
	}
    }
}

// Mutools RPC interface
#[tarpc::service]
pub trait World {

    // Misc API
    async fn get_version() -> String;
    async fn init_transaction() -> TransactionDescriptor;
    async fn commit_transaction(desc: TransactionDescriptor) -> ();
    
    // Project API
    async fn get_project_descriptors() -> Vec<ProjectDescriptor>;
    async fn get_sound_descriptors(desc : ProjectDescriptor) -> Vec<SoundDescriptor>;
    async fn get_buffer_descriptors(desc : SoundDescriptor) -> Vec<BufferDescriptor>;

    //async fn make_project() -> ProjectDescriptor;
    //async fn make_sound() -> SoundDescriptor;
    //async fn make_buffer() -> BufferDescriptor;

    // Transform API

    // Playback API
}

pub trait MutoolsRpcHandlerBase {

    fn get_version(&mut self, _ : context::Context) -> String;
    fn init_transaction(&mut self, _ : context::Context) -> TransactionDescriptor;
    fn commit_transaction(&mut self, _ : context::Context, desc: TransactionDescriptor) -> ();
    
    fn get_project_descriptors(&mut self, _ : context::Context) -> Vec<ProjectDescriptor>;
    fn get_sound_descriptors(&mut self, _ : context::Context, desc : ProjectDescriptor) -> Vec<SoundDescriptor>;
    fn get_buffer_descriptors(&mut self, _ : context::Context, desc : SoundDescriptor) -> Vec<BufferDescriptor>;

}

pub trait MutoolsRpcHandler: MutoolsRpcHandlerBase + Send + Sync {}

impl<T: MutoolsRpcHandlerBase + Send + Sync> MutoolsRpcHandler for T {}

pub struct MutoolsRpcHandlerProxy {
    handler: std::sync::Arc<std::sync::Mutex<Box<dyn MutoolsRpcHandler>>>
}

impl MutoolsRpcHandlerProxy {
    pub fn new(handler: Box<dyn MutoolsRpcHandler>) -> Self {
	Self {
	    handler: std::sync::Arc::new(std::sync::Mutex::new(handler)),
	}
    }
}
