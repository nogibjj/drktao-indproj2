import unittest
import sqlite3
from mylib.query import create_db, read_db, update_db, delete_tb, Query1, Query2
from mylib.transform_load import load
from mylib.extract import extract


class testFDBFunctions(unittest.TestCase):
    def test_create_db(self):
        result = create_db()
        self.assertEqual(result, "Success", "Failed to create table")

    def test_read_db(self):
        extract()
        load()
        conn = sqlite3.connect("GroceryDB.db")
        result = read_db(conn)
        self.assertEqual(result, "Success", "Failed to read table")

    def test_update_db(self):
        extract()
        load()
        conn = sqlite3.connect("GroceryDB.db")
        result = update_db(conn)
        self.assertEqual(result, "Success", "Failed to update table")

    def test_delete_tb(self):
        extract()
        load()
        result = delete_tb()
        self.assertEqual(result, "Success", "Failed to delete table")

    def test_Query1(self):
        extract()
        load()
        result = Query1()
        self.assertEqual(result, "Success", "Failed to retrieve data")

    def test_Query2(self):
        extract()
        load()
        result = Query2()
        self.assertEqual(result, "Success", "Failed to retrieve data")


if __name__ == "__main__":
    unittest.main()
