mod db;
mod models;
mod commands;
mod config;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::models::{Item, Point, Store};
use crate::commands::{create_store, open_store, get_store_list, create_space, get_spaces};
use crate::config::initialize_config;
use config::ConfigState;
use rusqlite::Connection;
use tauri::State;

struct AppState {
    temp_drawing: Mutex<Vec<Vec<(f32, f32)>>>,
}

#[tauri::command]
fn receive_drawing(state: State<Arc<AppState>>, shapes: Vec<Vec<(f32, f32)>>) {
    let mut drawing = state.temp_drawing.lock().unwrap();
    *drawing = shapes;
}

#[tauri::command]
fn send_drawing(state: State<Arc<AppState>>) -> Vec<Vec<(f32, f32)>> {
    let drawing = state.temp_drawing.lock().unwrap();
    drawing.clone()
}

// #[tauri::command]
// fn create_store(name: &str, space_id: i64, x: i32, y: i32) -> Result<i64, String> {
//     let conn = Connection::open("shapes.db").map_err(|e| e.to_string())?;

//     match conn.execute(
//         "INSERT INTO stores (name, space_id, x, y) VALUES (?1, ?2, ?3, ?4)", 
//         (name, space_id, x, y)
//     ) {
//         Ok(_) => {
//             Ok(conn.last_insert_rowid())
//         }
//         Err(e) => {
//             Err(format!("Failed to create store: {}", e))
//         }
//     }
// }


#[tauri::command]
fn fetch_store(id: i64) -> Result<Store, String> {
    let conn = Connection::open("shapes.db").map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT * 
        FROM stores 
        WHERE id = ?1"
    ).map_err(|e| e.to_string())?;

    let store_result = stmt.query_row([id], |row| {
        Ok(Store {
            id: row.get(0)?,
            name: row.get(2)?,
            location: Point {
                x: row.get(3)?,
                y: row.get(4)?,
            }
        })
    });

    match store_result {
        Ok(store) => Ok(store),
        Err(e) => {
            eprintln!("Query error: {}", e);
            Err(format!("Store with id {} not found", id))
        }
    }
}

#[tauri::command]
fn fetch_all_stores(space_id: i64) -> Result<Vec<Store>, String> {
    let conn = Connection::open("shapes.db").map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT * 
        FROM stores 
        WHERE space_id = ?1"
    ).map_err(|e| e.to_string())?;

    let store_iter = stmt.query_map([space_id], |row| {
        Ok(Store {
            id: row.get(0)?,
            name: row.get(2)?,
            location: Point {
                x: row.get(3)?,
                y: row.get(4)?,
            }
        })
    }).map_err(|e| e.to_string())?;

    let mut stores = Vec::new();

    for store in store_iter {
        match store {
            Ok(store) => stores.push(store),
            Err(e) => {
                eprintln!("Row error: {}", e);
                return Err("Error processing row".to_string());
            }
        }
    }

    Ok(stores)
}

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
        .manage(Mutex::new(config_state)) // Initialize and manage state
        .setup(initialize_config)
        .manage(Arc::new(AppState {
            temp_drawing: Mutex::new(Vec::new()), 
        }))
        .invoke_handler(tauri::generate_handler![create_store, open_store, get_store_list, create_space, get_spaces, receive_drawing, send_drawing, fetch_store, fetch_all_stores, add_item, fetch_item])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
