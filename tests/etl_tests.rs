use derek_tao_sqlite::{extract, load};

#[test]
fn test_extract() {
    let url =
        "https://raw.githubusercontent.com/Barabasi-Lab/GroceryDB/main/data/GroceryDB_IgFPro.csv";
    let file_path = "data/GroceryDB_IgFPro.csv";

    extract(url, file_path);

    assert!(std::fs::metadata(file_path).is_ok());
}

#[test]
fn test_load() {
    let dataset = "data/GroceryDB_IgFPro.csv";
    let result = load(dataset);

    assert_eq!(result.unwrap(), "GroceryDB.db");
}

