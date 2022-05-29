#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
async fn ping_rpc() {
    let rpc_client = {
	let res = mutools_rpc::client::MutoolsRpcClient::new(5000);
	match res.await {
	    Ok(client) => client,
	    _ => panic!("Mutools Client could not make contact with server!"),
	    }
    };
    /*
    let nd = rpc_client.get_client().get_project_descriptors(mutools_rpc::get_current_context()).await;
    match nd {
	Ok(n) => println!("Got: {}", n),
	Err(_) => println!("Rpc returned an error"),
    }
    */
}

fn main() {    
    
    tauri::Builder::default()
	.invoke_handler(tauri::generate_handler![ping_rpc])
	.run(tauri::generate_context!())
	.expect("error while running tauri application");
}
