use tauri::Manager;


#[tauri::command] 
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

#[async_std::main]
fn main() {
    tauri::Builder::default() 
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let mut mac_ev_observer = lib::MacEventObserver::new();
            mac_ev_observer.start();
            app.manage(mac_ev_observer);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
