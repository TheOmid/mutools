pub mod client;
pub mod interface;
pub mod server;

pub fn get_current_context() -> tarpc::context::Context {
    tarpc::context::current()
}
