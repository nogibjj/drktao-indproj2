use derek_tao_sqlite::{create_db, delete_tb, extract, load, read_db, update_db};
use rusqlite::Connection;
fn main() {
    let url =
        "https://raw.githubusercontent.com/Barabasi-Lab/GroceryDB/main/data/GroceryDB_IgFPro.csv";
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
