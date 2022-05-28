use mutools_rpc::interface::{
    MutoolsRpcHandler,
    MutoolsRpcHandlerBase,
    TransactionDescriptor,
    ProjectDescriptor,
    SoundDescriptor,
    BufferDescriptor
};

use tarpc::context;

pub struct MutoolsServer {
}

impl MutoolsServer {
    pub fn new() -> Self {
	Self {}
    }
}

impl MutoolsRpcHandlerBase for MutoolsServer {

    fn get_version(&mut self, _ : context::Context) -> String {
	"0.1.0".into()
    }
	
    fn init_transaction(&mut self, _ : context::Context) -> TransactionDescriptor {
	TransactionDescriptor::new()
    }

    fn commit_transaction(&mut self, _ : context::Context, desc: TransactionDescriptor) -> () {
	()
    }
    
    fn get_project_descriptors(&mut self, _ : context::Context) -> Vec<ProjectDescriptor> {
	vec![].into()
    }
    
    fn get_sound_descriptors(&mut self, _ : context::Context, desc : ProjectDescriptor) -> Vec<SoundDescriptor> {
	vec![].into()
    }

    fn get_buffer_descriptors(&mut self, _ : context::Context, desc : SoundDescriptor) -> Vec<BufferDescriptor> {
	vec![].into()
    }
    
}

