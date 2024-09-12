mod db;
mod models;
mod commands;
mod config;

use std::path::PathBuf;
use std::sync::Mutex;
use crate::models::Item;
use crate::commands::{create_store, open_store, get_store_list, create_space, get_space, get_space_list, create_bin};
use crate::config::initialize_config;
use config::ConfigState;
use rusqlite::Connection;

#[tauri::command]
fn add_item(store_id: i64, name: &str, quantity: i32, notes: &str) -> Result<i64, String> {
    let conn = Connection::open("shapes.db").map_err(|e| e.to_string())?;

    match conn.execute("
        INSERT INTO items (store_id, name, quantity) VALUES (?1, ?2, ?3)", 
        (store_id, name, quantity)
    ) {
        Ok(_) => {
            Ok(conn.last_insert_rowid())
        }
        Err(e) => {
            eprintln!("Query error: {}", e);
            Err(format!("Failed to add item: {}", e.to_string()))
        }
    }
}

#[tauri::command]
fn fetch_item(id: i64) -> Result<Item, String> {
    let conn = Connection::open("shapes.db").map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT * 
        FROM items 
        WHERE id = ?1"
    ).map_err(|e| e.to_string())?;

    match stmt.query_row([id], |row| {
        Ok(Item {
            id: row.get(0)?,
            store_id: row.get(1)?,
            name: row.get(2)?,
            quantity: row.get(3)?
        })
    }){
        Ok(item) => Ok(item),
        Err(e) => {
            eprintln!("Query error: {}", e);
            Err(format!("Store with id {} not found", id))
        }
    }
}

fn main() {
    let config_state = ConfigState {
        config_path: PathBuf::new(),
    };

    tauri::Builder::default()
        .manage(Mutex::new(config_state))
        .setup(initialize_config)
        .invoke_handler(tauri::generate_handler![create_store, open_store, get_store_list, create_space, get_space, get_space_list, create_bin, add_item, fetch_item])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
