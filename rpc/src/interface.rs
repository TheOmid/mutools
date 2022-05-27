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

// Mutools RPC interface
#[tarpc::service]
pub trait World {

    // Misc API
    async fn get_version() -> String;

    // Project API
    async fn get_num_project_descriptors() -> usize;

    async fn get_project_descriptors() -> Vec<ProjectDescriptor>;
    async fn get_sound_descriptors(desc : ProjectDescriptor) -> Vec<SoundDescriptor>;
    async fn get_buffer_descriptors(desc : SoundDescriptor) -> Vec<BufferDescriptor>;

    // Transform API


    // Playback API
}

pub trait MutoolsRpcHandlerBase {

    fn get_version(&self, _ : context::Context) -> String;
    fn get_num_project_descriptors(&self, _ : context::Context) -> usize;

    fn get_project_descriptors(&self, _ : context::Context) -> Vec<ProjectDescriptor>;
    fn get_sound_descriptors(&self, desc : ProjectDescriptor, _ : context::Context) -> Vec<SoundDescriptor>;
    fn get_buffer_descriptors(&self, desc : SoundDescriptor, _ : context::Context) -> Vec<BufferDescriptor>;

}

pub trait MutoolsRpcHandler: MutoolsRpcHandlerBase + Send + Sync {}
