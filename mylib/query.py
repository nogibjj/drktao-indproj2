import sqlite3

def create_db():
    conn = sqlite3.connect("GroceryDB.db")
    cursor = conn.cursor()
    cursor.execute(
        """CREATE TABLE IF NOT EXISTS GroceryDB
                      (general_name INTEGER PRIMARY KEY,
                      count_products INTEGER,
                      ingred_FPro FLOAT,
                      avg_FPro_products FLOAT,
                      avg_distance_root FLOAT,
                      ingred_normalization_term	FLOAT,
                      semantic_tree_name CHAR,
                      semantic_tree_node CHAR)"""
    )
    conn.commit()
    print("Table created")
    return "Success"

def read_db(conn):
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM GroceryDB LIMIT 5")
    rows = cursor.fetchall()

    for row in rows:
        print(row)
    print("Table read")
    return "Success"

def update_db(conn):
    cursor = conn.cursor()
    cursor.execute("UPDATE GroceryDB SET semantic_tree_name = 'coffee' WHERE general_name = 'arabica coffee'")
    conn.commit()
    print("Table updated")
    return "Success"

def delete_tb():
    conn = sqlite3.connect("GroceryDB.db")
    cursor = conn.cursor()
    cursor.execute(
        "DROP TABLE IF EXISTS GroceryDB"
    )
    conn.commit()
    conn.close()
    print("Table dropped")
    return "Success"

def Query1():
    conn = sqlite3.connect("GroceryDB.db")
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM GroceryDB WHERE general_name LIKE '%coffee%' LIMIT 5")
    rows = cursor.fetchall()

    for row in rows:
        print(row)
    print("rows retrieved")
    return "Success"

def Query2():
    conn = sqlite3.connect("GroceryDB.db")
    cursor = conn.cursor()
    cursor.execute("SELECT general_name FROM GroceryDB WHERE count_products>20 LIMIT 5")
    rows = cursor.fetchall()

    for row in rows:
        print(row)
    print("rows retrieved")
    return "Success"