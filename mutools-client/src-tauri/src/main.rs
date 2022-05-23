#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
    
#[tauri::command]
fn DispatchPing() -> String {
    println!("I was invoked from JS!");
    "HELLO FROM RUST".into()
}

fn main() {
  tauri::Builder::default()
	.invoke_handler(tauri::generate_handler![DispatchPing])
	.run(tauri::generate_context!())
	.expect("error while running tauri application");
}
