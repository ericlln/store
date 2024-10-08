use std::{path::{Path, PathBuf}, sync::Mutex};
use rusqlite::{Connection, Params, Result};
use tauri::State;
use crate::{config::{read_config, remember_store, retrieve_store_path, ConfigState}, db::create_tables, models::{Bin, Item, Point, Space, Store}};

#[tauri::command]
pub fn create_store(state: State<'_, Mutex<ConfigState>>, name: &str, path: &str) -> Result<(), String> {
    let db_path = &Path::new(path).join(format!("{}.db", name));

    if db_path.exists() {
        return Err(format!("A file with name {} already exists", db_path.display().to_string()))
    }

    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    
    if let Err(e) = create_tables(&conn) {
        return Err(format!("Error creating tables: {e}"));
    }
    
    if let Err(e) = remember_store(&state, name, db_path.display().to_string().as_str()) {
        return Err(format!("Error adding store to config: {e}"));
    }

    Ok(())
}

#[tauri::command]
pub fn open_store(state: State<'_, Mutex<ConfigState>>, name: &str) -> Result<String, String> {
    let config_json = read_config(&state).map_err(|e| e.to_string())?;

    if let Some(stores) = config_json.get("stores").and_then(|v| v.as_object()) {
        if let Some(path) = stores.get(name).and_then(|v| v.as_str()) {
            let path_buf = PathBuf::from(path);
            if !path_buf.exists() {
                return Err(format!("Store {name} does not exist at {path}"));
            }

            // Return the parent directory rather than the full path
            if let Some(parent_dir) = path_buf.parent() {
                return Ok(parent_dir.display().to_string());
            } else {
                return Err(format!("Unable to get the parent directory of {path}"));
            }
        }
    }

    Err(format!("Store {name} not found"))
}

#[tauri::command]
pub fn get_store_list(state: State<'_, Mutex<ConfigState>>) -> Result<Vec<Store>, String> {
    let config_json = read_config(&state).map_err(|e| e.to_string())?;

    let mut store_list = Vec::new();

    if let Some(stores) = config_json.get("stores").and_then(|v| v.as_object()) {
        for kvp in stores.iter() {
            if let Some(path) = kvp.1.as_str() {
                store_list.push(Store {name: kvp.0.to_string(), path: path.to_string(), available: PathBuf::from(path).exists()});
            }
        }
    }

    Ok(store_list)
}

#[tauri::command]
pub fn create_space(state: State<'_, Mutex<ConfigState>>, store_name: &str, name: &str, drawing: Vec<Vec<Point>>) -> Result<i64, String> {
    let path = retrieve_store_path(&state, store_name)?;
    let conn = Connection::open(&path).map_err(|e| e.to_string())?;
    let drawing_json = serde_json::to_string(&drawing).map_err(|e| e.to_string())?;

    match conn.execute(
        "INSERT INTO spaces (name, drawing_json) VALUES (?1, ?2)",
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
pub fn get_space(state: State<'_, Mutex<ConfigState>>, store_name: &str, space_id: i64) -> Result<Space, String> {
    let path = retrieve_store_path(&state, store_name).map_err(|e| e.to_string())?;
    let conn = Connection::open(path).map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT id, name, drawing_json
        FROM spaces
        WHERE id = ?1"
    ).map_err(|e| e.to_string())?;

    let space = stmt.query_row([space_id], |row| {
        let id: i64 = row.get(0)?;
        let name: String = row.get(1)?;
        let drawing_json: String = row.get(2)?;
        let drawing: Result<Vec<Vec<Point>>, serde_json::Error> = serde_json::from_str(&drawing_json);

        match drawing {
            Ok(drawing) => Ok(Space { id, name, drawing: Some(drawing) }),
            Err(e) => Err(rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))),
        }
    });

    space.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_space_list(state: State<'_, Mutex<ConfigState>>, store_name: &str) -> Result<Vec<Space>, String> {
    let path = retrieve_store_path(&state, store_name)?;
    let conn = Connection::open(path).map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT id, name, drawing_json
        FROM spaces"
    ).map_err(|e| e.to_string())?;

    let iter = stmt.query_map([], |row| {
        let id: i64 = row.get(0)?;
        let name: String = row.get(1)?;
        Ok(Space { id, name, drawing: None })
    }).map_err(|e| e.to_string())?;

    let spaces: Vec<Space> = iter.collect::<Result<Vec<Space>>>().map_err(|e| e.to_string())?;

    Ok(spaces)
}

#[tauri::command]
pub fn create_bin(state: State<'_, Mutex<ConfigState>>, store_name: &str, space_id: i64, name: &str, x: f64, y: f64) -> Result<(), String> {
    let path = retrieve_store_path(&state, store_name)?;
    let conn = Connection::open(path).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO bins (space_id, name, x, y) VALUES (?1, ?2, ?3, ?4)",
        (space_id, name, x, y)
    ).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_bin_list(state: State<'_, Mutex<ConfigState>>, store_name: &str, space_id: i64) -> Result<Vec<Bin>, String> {
    let path = retrieve_store_path(&state, store_name)?;
    let conn = Connection::open(path).map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT id, space_id, name, x, y
        FROM bins
        WHERE space_id = ?1"
    ).map_err(|e| e.to_string())?;

    let iter = stmt.query_map([space_id], |row| {
        let id: i64 = row.get(0)?;
        let space_id: i64 = row.get(1)?;
        let name: String = row.get(2)?;
        let x: f64 = row.get(3)?;
        let y: f64 = row.get(4)?;

        Ok(Bin { id, space_id, name, location: Point {x, y} })
    }).map_err(|e| e.to_string())?;

    let bins: Vec<Bin> = iter.collect::<Result<Vec<Bin>>>().map_err(|e| e.to_string())?;

    Ok(bins)
}

#[tauri::command]
pub fn create_item(state: State<'_, Mutex<ConfigState>>, store_name: &str, space_id: i64, bin_id: i64, name: &str) -> Result<(), String> {
    let path = retrieve_store_path(&state, store_name)?;
    let conn = Connection::open(path).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO items (space_id, bin_id, name) VALUES (?1, ?2, ?3)",
        (space_id, bin_id, name)
    ).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_item_list(state: State<'_, Mutex<ConfigState>>, store_name: &str, bin_id: Option<i64>) -> Result<Vec<Item>, String> {
    let path = retrieve_store_path(&state, store_name)?;
    let conn = Connection::open(path).map_err(|e| e.to_string())?;

    let sql = if bin_id.is_some() {
        "SELECT i.id, i.space_id, bin_id, i.name, quantity, notes, b.name, s.name
        FROM items as i
        LEFT JOIN bins as b ON i.bin_id = b.id 
        LEFT JOIN spaces as s ON i.space_id = s.id
        WHERE bin_id = ?1"
    } else {
        "SELECT i.id, i.space_id, bin_id, i.name, quantity, notes, b.name, s.name
        FROM items as i
        LEFT JOIN bins as b ON i.bin_id = b.id 
        LEFT JOIN spaces as s ON i.space_id = s.id"
    };

    let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;

    let params: &[&(dyn rusqlite::ToSql)] = match bin_id {
        Some(ref bin_id_value) => &[bin_id_value as &(dyn rusqlite::ToSql)],
        None => &[],
    };

    let iter = stmt.query_map(params, |row| {
        let id: i64 = row.get(0)?;
        let space_id: i64 = row.get(1)?;
        let bin_id: i64 = row.get(2)?;
        let name: String = row.get(3)?;
        let quantity: Option<i32> = row.get(4)?;
        let notes: Option<String> = row.get(5)?;
        let bin_name: Option<String> = row.get(6)?;
        let space_name: Option<String> = row.get(7)?;

        Ok(Item { id, space_id, bin_id, name, quantity, notes, space_name, bin_name })
    }).map_err(|e| e.to_string())?;

    let items: Vec<Item> = iter.collect::<Result<Vec<Item>>>().map_err(|e| e.to_string())?;

    Ok(items)
}