use rusqlite::Connection;

pub fn create_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
    let query = "
    CREATE TABLE IF NOT EXISTS spaces (
        id INTEGER PRIMARY KEY,  
        name TEXT NOT NULL,
        drawing_json TEXT
    );

    CREATE TABLE IF NOT EXISTS stores (
        id INTEGER PRIMARY KEY,
        space_id INTEGER,
        name TEXT NOT NULL,
        x REAL,
        y REAL,

        FOREIGN KEY (space_id) REFERENCES spaces(id) ON DELETE SET NULL
    );
    
    CREATE TABLE IF NOT EXISTS items (
        id INTEGER PRIMARY KEY,
        store_id INTEGER,
        name TEXT NOT NULL,
        quantity INTEGER,
        notes TEXT,

        FOREIGN KEY (store_id) REFERENCES stores(id) ON DELETE SET NULL
    )";

    conn.execute_batch(query)?;
    Ok(())
}