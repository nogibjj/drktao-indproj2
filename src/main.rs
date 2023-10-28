use std::fs::File;
use reqwest::blocking::get;
use rusqlite::{params, Connection, Result};

fn extract(url: &str, file_path: &str) -> Result<String> {
    let mut response = get(url)?;
    let mut file = File::create(file_path)?;
    response.copy_to(&mut file)?;
    Ok(file_path.to_string())
}

fn load(dataset: &str) -> Result<String> {
    //prints the full working directory and path
    println!("{:?}", std::env::current_dir().unwrap());

    let mut payload = csv::Reader::from_path(dataset)?;
    let conn = Connection::open("GroceryDB.db")?;
    conn.execute("DROP TABLE IF EXISTS GroceryDB", [])?;
    conn.execute("CREATE TABLE GroceryDB (id,general_name, count_products, ingred_FPro, avg_FPro_products, avg_distance_root, ingred_normalization_term, semantic_tree_name, semantic_tree_node)", [])?;
    let mut stmt = conn.prepare("INSERT INTO GroceryDB VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")?;
    for result in payload.records() {
        let record = result?;
        stmt.execute(&[&record[0], &record[1], &record[2], &record[3], &record[4], &record[5], &record[6], &record[7], &record[8]])?;
    }
    Ok("GroceryDB.db".to_string())
}

fn create_db() -> Result<String> {
    let conn = Connection::open("GroceryDB.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS GroceryDB (
            general_name INTEGER PRIMARY KEY,
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

fn read_db(conn: &Connection) -> Result<String> {
    let mut stmt = conn.prepare("SELECT * FROM GroceryDB LIMIT 5")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<usize, i64>(0)?,
            row.get::<usize, i64>(1)?,
            row.get::<usize, f64>(2)?,
            row.get::<usize, f64>(3)?,
            row.get::<usize, f64>(4)?,
            row.get::<usize, f64>(5)?,
            row.get::<usize, String>(6)?,
            row.get::<usize, String>(7)?,
        ))
    })?;

    for row in rows {
        println!("{:?}", row?);
    }
    println!("Table read");
    Ok("Success".to_string())
}

fn update_db(conn: &Connection) -> Result<String> {
    conn.execute(
        "UPDATE GroceryDB SET semantic_tree_name = 'coffee' WHERE general_name = 'arabica coffee'",
        params![],
    )?;
    println!("Table updated");
    Ok("Success".to_string())
}

fn delete_tb() -> Result<String> {
    let conn = Connection::open("GroceryDB.db")?;
    conn.execute("DROP TABLE IF EXISTS GroceryDB", params![])?;
    println!("Table dropped");
    Ok("Success".to_string())
}

fn main() {
    let url = "https://raw.githubusercontent.com/Barabasi-Lab/GroceryDB/main/data/GroceryDB_IgFPro.csv";
    let file_path = "data/GroceryDB_IgFPro.csv";
    println!("Extracting data...");
    if let Err(e) = extract(url, file_path) {
        println!("Error: {}", e);
    }
    println!("Loading data...");
    if let Err(e) = load(file_path) {
        println!("Error: {}", e);
    }
    println!("Querying Data...");
    if let Err(e) = create_db() {
        println!("Error: {}", e);
    }
    if let Err(e) = read_db(&Connection::open("GroceryDB.db").unwrap()) {
        println!("Error: {}", e);
    }
    if let Err(e) = update_db(&Connection::open("GroceryDB.db").unwrap()) {
        println!("Error: {}", e);
    }
    if let Err(e) = delete_tb() {
        println!("Error: {}", e);
}
    
    

}