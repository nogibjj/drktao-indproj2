use reqwest::blocking::get;
use rusqlite::{params, Connection, Result};
use std::fs::File;

pub fn extract(url: &str, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut response = get(url)?;
    let mut file = File::create(file_path)?;
    response.copy_to(&mut file)?;
    Ok(file_path.to_string())
}

pub fn load(dataset: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Prints the full working directory and path
    println!("{:?}", std::env::current_dir().unwrap());

    let payload = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path(dataset);

    let conn = Connection::open("GroceryDB.db")?;
    conn.execute("DROP TABLE IF EXISTS GroceryDB", [])?;
    conn.execute(
        "CREATE TABLE GroceryDB (
            id INTEGER PRIMARY KEY,
            general_name TEXT,
            count_products INTEGER,
            ingred_FPro REAL,
            avg_FPro_products REAL,
            avg_distance_root REAL,
            ingred_normalization_term REAL,
            semantic_tree_name TEXT,
            semantic_tree_node TEXT
        )",
        [],
    )?;

    let mut stmt = conn.prepare(
        "INSERT INTO GroceryDB (
            id,
            general_name,
            count_products,
            ingred_FPro,
            avg_FPro_products,
            avg_distance_root,
            ingred_normalization_term,
            semantic_tree_name,
            semantic_tree_node
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
    )?;

    for result in payload?.records() {
        let record = result?;
        stmt.execute([
            &record[0], &record[1], &record[2], &record[3], &record[4], &record[5], &record[6],
            &record[7], &record[8],
        ])?;
    }

    Ok("GroceryDB.db".to_string())
}

pub fn create_db() -> Result<String> {
    let conn = Connection::open("GroceryDB.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS GroceryDB (
            id INTEGER PRIMARY KEY,
            general_name TEXT,
            count_products INTEGER,
            ingred_FPro FLOAT,
            avg_FPro_products FLOAT,
            avg_distance_root FLOAT,
            ingred_normalization_term FLOAT,
            semantic_tree_name CHAR,
            semantic_tree_node CHAR
        )",
        params![],
    )?;
    println!("Table created");
    Ok("Success".to_string())
}

pub fn read_db(conn: &Connection) -> Result<String> {
    let mut stmt = conn.prepare("SELECT * FROM GroceryDB LIMIT 5")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<usize, i64>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, f64>(2)?,
            row.get::<usize, f64>(3)?,
            row.get::<usize, f64>(4)?,
            row.get::<usize, f64>(5)?,
            row.get::<usize, f64>(6)?,
            row.get::<usize, String>(7)?,
            row.get::<usize, String>(8)?,
        ))
    })?;

    for row in rows {
        println!("{:?}", row?);
    }
    println!("Table read");
    Ok("Success".to_string())
}

pub fn update_db(conn: &Connection) -> Result<String> {
    conn.execute(
        "UPDATE GroceryDB SET semantic_tree_name = 'coffee' WHERE general_name = 'arabica coffee'",
        params![],
    )?;
    println!("Table updated");
    Ok("Success".to_string())
}

pub fn delete_tb() -> Result<String> {
    let conn = Connection::open("GroceryDB.db")?;
    conn.execute("DROP TABLE IF EXISTS GroceryDB", params![])?;
    println!("Table dropped");
    Ok("Success".to_string())
}
