use rusqlite::{Connection, Result};

pub fn create_table() -> Result<()> {
    let conn = Connection::open("my_database.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
                  id INTEGER PRIMARY KEY,
                  name TEXT NOT NULL
                  )",
        [],
    )?;

    Ok(())
}

pub fn insert_item(name: &str) -> Result<()> {
    let conn = Connection::open("my_database.db")?;
    conn.execute("INSERT INTO items (name) VALUES (?1)", [name])?;
    Ok(())
}

pub fn read_items() -> Result<()> {
    let conn = Connection::open("my_database.db")?;
    let mut stmt = conn.prepare("SELECT id, name FROM items")?;
    let items_iter = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?;

    for item in items_iter {
        println!("Found item {:?}", item?);
    }

    Ok(())
}

pub fn update_item(id: i32, new_name: &str) -> Result<()> {
    let conn = Connection::open("my_database.db")?;
    conn.execute("UPDATE items SET name = ?1 WHERE id = ?2", [new_name, &id.to_string()])?;
    Ok(())
}

pub fn delete_item(id: i32) -> Result<()> {
    let conn = Connection::open("my_database.db")?;
    conn.execute("DELETE FROM items WHERE id = ?1", [&id.to_string()])?;
    Ok(())
}
