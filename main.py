from mylib.extract import extract
from mylib.query import create_db, read_db, update_db, delete_tb, Query1, Query2
from mylib.transform_load import load

import sqlite3

print("Extracting data...")
extract()

print("Loading data...")
load()

print("Querying data...")
create_db()
print()
read_db(sqlite3.connect("GroceryDB.db"))
print()
update_db(sqlite3.connect("GroceryDB.db"))
print()
delete_tb()
print()
load()
print()
Query1()
print()
Query2()
