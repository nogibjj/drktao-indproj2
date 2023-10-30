use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rusqlite::{Connection, Result};

//load the csv file and insert into a new sqlite3 database
fn load(dataset: &str) -> Result<String> {
    //prints the full working directory and path
    println!("{:?}", std::env::current_dir().unwrap());
    let mut payload = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path(dataset)?;
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
    for result in payload.records() {
        let record = result?;
        stmt.execute(&[
            &record[0],
            &record[1],
            &record[2],
            &record[3],
            &record[4],
            &record[5],
            &record[6],
            &record[7],
            &record[8],
        ])?;
    }
    Ok("GroceryDB.db".to_string())
}
