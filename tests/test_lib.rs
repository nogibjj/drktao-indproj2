use derek_tao_sqlite::{create_db, delete_tb, extract, load};

#[test]
fn test_extract() {
    let url =
        "https://raw.githubusercontent.com/Barabasi-Lab/GroceryDB/main/data/GroceryDB_IgFPro.csv";
    let file_path = "data/GroceryDB_IgFPro.csv";
    assert_eq!(extract(url, file_path).unwrap(), file_path.to_string());
}

#[test]
fn test_load() {
    let dataset = "data/GroceryDB_IgFPro.csv";
    assert_eq!(load(dataset).unwrap(), "GroceryDB.db".to_string());
}

#[test]
fn test_create_db() {
    assert_eq!(create_db().unwrap(), "Success".to_string());
}

#[test]
fn test_delete_tb() {
    assert_eq!(delete_tb().unwrap(), "Success".to_string());
}
