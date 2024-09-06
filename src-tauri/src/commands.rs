use std::{fs::File, io::Read, path::Path, sync::Mutex};
use rusqlite::Connection;
use serde_json::Value;
use tauri::State;
use crate::{config::{remember_store, ConfigState}, db::create_tables, models::Space};

#[tauri::command]
pub fn create_store(state: State<'_, Mutex<ConfigState>>, name: &str, path: &str) -> Result<(), String> {
    let db_path = &Path::new(path).join(format!("{}.db", name));

    if db_path.exists() {
        return Err(format!("A file with name {} already exists", db_path.display().to_string()))
    }

    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    
    if let Err(e) = create_tables(&conn) {
        eprintln!("Error creating tables: {}", e);
        return Err(e.to_string());
    }
    
    if let Err(e) = remember_store(state, name, db_path.display().to_string().as_str()) {
        eprintln!("{}", e);
        return Err(e);
    }

    Ok(())
}

#[tauri::command]
pub fn open_store(state: State<'_, Mutex<ConfigState>>, name: &str) -> Result<(), String> {
    let state = state.lock().unwrap();
    let config_path = &state.config_path;

    let mut file = File::open(&config_path).map_err(|e| format!("Failed to open config file: {}", e))?;
    let mut data = String::new();
    file.read_to_string(&mut data).map_err(|e| format!("Failed to read config file: {}", e))?;

    let mut json_data: Value = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

    if let Some(stores) = json_data.get_mut("stores").and_then(|v| v.as_array_mut()) {
        for store in stores.iter() {
            if let Some(obj) = store.as_object() {
                if obj.contains_key(name) {
                    return Ok(());
                }
            }
        }
    }

    Err(format!("Store {} not found", name))
}

#[tauri::command]
pub fn new_space(name: &str, drawing: Vec<Vec<(f32, f32)>>) -> Result<i64, String> {
    let conn = Connection::open("shapes.db").map_err(|e| e.to_string())?;
    let drawing_json = serde_json::to_string(&drawing).map_err(|e| e.to_string())?;

    match conn.execute(
        "INSERT INTO spaces (name, drawing) VALUES (?1, ?2)",
        (name, drawing_json)
    ) {
        Ok(_) => {
            Ok(conn.last_insert_rowid())
        }
        Err(e) => {
            Err(format!("Failed to save drawing: {}", e))
        }
    }
}

#[tauri::command]
pub fn get_store_list(state: State<'_, Mutex<ConfigState>>) -> Result<Vec<String>, String> {
    let state = state.lock().unwrap();
    let config_path = &state.config_path;

    let mut file = File::open(&config_path).map_err(|e| format!("Failed to open config file: {}", e))?;
    let mut data = String::new();
    file.read_to_string(&mut data).map_err(|e| format!("Failed to read config file: {}", e))?;

    let mut json_data: Value = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

    let mut store_list = Vec::new();
    if let Some(stores) = json_data.get_mut("stores").and_then(|v| v.as_array_mut()) {
        for store in stores.iter() {
            if let Some(obj) = store.as_object() {
                for key in obj.keys() {
                    store_list.push(key.clone());
                }
            }
        }
    }

    Ok(store_list)
}

#[tauri::command]
pub fn get_space_list() -> Result<Vec<String>, String> {
    let conn = Connection::open("shapes.db").map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT name
        FROM spaces"
    ).map_err(|e| e.to_string())?;

    let iter = stmt.query_map([], |row| {
        Ok(row.get(1)?)
    }).map_err(|e| e.to_string())?;

    let names: Result<Vec<String>, String> = iter.collect::<Result<_, _>>().map_err(|e| e.to_string());
    names
}

#[tauri::command]
pub fn fetch_space(id: i64) -> Result<Space, String> {
    let conn = Connection::open("shapes.db").map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT *
        FROM STORES
        WHERE ID = ?1"
    ).map_err(|e| e.to_string())?;

    match stmt.query_row([id], |row| {
        let drawing_json: String = row.get(2)?;
        let drawing: Vec<Vec<(f32, f32)>> = serde_json::from_str(&drawing_json).unwrap(); //todo handle error

        Ok(Space {
            id: row.get(0)?,
            name: row.get(1)?,
            drawing,
        })
    }){
        Ok(space) => Ok(space),
        Err(e) => {
            eprintln!("Query error: {}", e);
            Err(format!("Space with id {} not found", id))
        }
    }
}