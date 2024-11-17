use tauri::http::StatusCode;
use tauri_plugin_http::reqwest;

struct Name {
    id: String,
    name: String,
}

#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Io(#[from] reqwest::Error)
}

impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}


#[tauri::command]
async fn add_name(name: &str) -> Result<String, Error> {
    format!("Hello, {}! You've been greeted from Rust!", name);
    let client = reqwest::Client::new();
    let add_status = client.post(format!("http://localhost:8081/add/{}", name))
        .send()
        .await?
        .status();

    if add_status != StatusCode::OK { 
        ()
    }

    let all_names = client.get("http://localhost:8081/getAll")
        .send()
        .await?
        .text().await?;

    Ok(all_names)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![add_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
