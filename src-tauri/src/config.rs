use std::error::Error;
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
                "stores": {},
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

pub fn read_config(state: &State<'_, Mutex<ConfigState>>) -> Result<Value, Box<dyn Error>> {
    let state = state.lock().unwrap();
    let config_path = &state.config_path;

    let mut file = File::open(&config_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let json_data: Value = serde_json::from_str(&data)?;

    Ok(json_data)
}

pub fn remember_store(state: &State<'_, Mutex<ConfigState>>, name: &str, path: &str) -> Result<(), String> {
    let mut config_json = read_config(state).map_err(|e| e.to_string())?;

    let state = state.lock().unwrap();
    let config_path = &state.config_path;

    if let Some(stores) = config_json.get_mut("stores").and_then(|v| v.as_object_mut()) {
        stores.insert(name.to_string(), Value::String(path.to_string()));
    } else {
        return Err(format!("Failed to save store {name} with path {path} into config").to_string());
    }

    write_json_to_file(&config_path, &config_json)
        .map_err(|e| format!("Failed to write JSON to file: {}", e))?;

    Ok(())
}

pub fn forget_store(state: &State<'_, Mutex<ConfigState>>, name: &str) ->Result<(), String> {
    let mut config_json = read_config(state).map_err(|e| format!("Error reading config: {e}"))?;

    if let Some(stores) = config_json.get_mut("stores").and_then(|v| v.as_object_mut()) {
        stores.remove(name);
    }

    Ok(())
}

pub fn retrieve_store_path(state: &State<'_, Mutex<ConfigState>>, store_name: &str) -> Result<String, String> {
    let config_json = read_config(state).map_err(|e| e.to_string())?;

    if let Some(stores) = config_json.get("stores").and_then(|v| v.as_object()) {
        if let Some(path) = stores.get(store_name).and_then(|v| v.as_str()) {
            return Ok(path.to_string());
        }
    }
        
    Err(format!("Store {store_name} not found"))
}

fn write_json_to_file(path: &PathBuf, data: &serde_json::Value) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data.to_string().as_bytes())?;
    Ok(())
}