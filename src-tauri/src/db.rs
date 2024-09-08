use rusqlite::Connection;

pub fn create_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
    let query = "
    CREATE TABLE IF NOT EXISTS spaces (
        id INTEGER PRIMARY KEY,  
        name TEXT NOT NULL,
        drawing_json TEXT
    );
    
    CREATE TABLE IF NOT EXISTS bins (
        id INTEGER PRIMARY KEY,
        space_id INTEGER,
        name TEXT NOT NULL,
        x REAL,
        y REAL,

        FOREIGN KEY (space_id) REFERENCES spaces(id) ON DELETE SET NULL
    );

    CREATE TABLE IF NOT EXISTS items (
        id INTEGER PRIMARY KEY,
        space_id INTEGER, 
        bin_id INTEGER,
        name TEXT NOT NULL,
        quantity INTEGER,

        FOREIGN KEY (space_id) REFERENCES spaces(id) ON DELETE SET NULL,
        FOREIGN KEY (bin_id) REFERENCES bind(id) ON DELETE SET NULL
    )";

    conn.execute_batch(query)?;
    Ok(())
}