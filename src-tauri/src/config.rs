use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use serde_json::{json, Value};
use tauri::{Manager, State};

#[derive(Default, Clone)]
pub struct ConfigState {
    pub config_path: PathBuf,
}

pub fn initialize_config(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(app_data_dir) = app.handle().path_resolver().app_data_dir() {
        if !app_data_dir.exists() {
            fs::create_dir_all(&app_data_dir)?;
        }

        let config_path = app_data_dir.join("config.json");

        if !config_path.exists() {
            let config = json!({
                "stores": [],
            });
    
            if let Err(e) = write_json_to_file(&config_path, &config) {
                eprintln!("Failed to write config file: {}", e);
                return Err(Box::new(e));
            }
        }

        // Save config path to be used later
        let state = app.state::<Mutex<ConfigState>>();
        let mut state = state.lock().unwrap();
        state.config_path = config_path;
    } else {
        return Err("Failed to resolve app data directory.".into()); // Return error if app data dir resolution fails
    }
    Ok(())
}

pub fn remember_store(state: State<'_, Mutex<ConfigState>>, name: &str, path: &str) -> Result<(), String> {
    let state = state.lock().unwrap();
    let config_path = &state.config_path;

    let mut file = File::open(&config_path).map_err(|e| format!("Failed to open config file: {}", e))?;
    let mut data = String::new();
    file.read_to_string(&mut data).map_err(|e| format!("Failed to read config file: {}", e))?;

    let mut json_data: Value = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

    if let Some(stores) = json_data.get_mut("stores").and_then(|v| v.as_array_mut()) {
        stores.push(json!({name: path}));
    } else {
        return Err("Failed to find or create the 'stores' array in the JSON data.".to_string());
    }

    write_json_to_file(&config_path, &json_data)
        .map_err(|e| format!("Failed to write JSON to file: {}", e))?;

    Ok(())
}

fn write_json_to_file(path: &PathBuf, data: &serde_json::Value) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data.to_string().as_bytes())?;
    Ok(())
}
