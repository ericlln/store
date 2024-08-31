use rusqlite::Connection;

use crate::models::{Item, Space};

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