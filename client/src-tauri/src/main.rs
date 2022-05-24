#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {

    tauri::async_runtime::spawn(async move {
	let rpc_client = {
	    let res = mutools_rpc::client::MutoolsRpcClient::new(5000);
	    match res {
		Ok(client) => client,
		_ => panic!("Mutools Client could not make contact with server!"),
	    }
	};
	
	let nd = rpc_client.get_client().get_num_project_descriptors(mutools_rpc::get_current_context()).await;
	match nd {
	    Ok(n) => println!("Got: {}", n),
	    Err(_) => println!("Rpc returned an error"),
	}
    });
    
    
    tauri::Builder::default()
	.run(tauri::generate_context!())
	.expect("error while running tauri application");
}
