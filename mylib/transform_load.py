"""
Transforms and Loads data into the local SQLite3 database
"""
import sqlite3
import csv
import os

#load the csv file and insert into a new sqlite3 database
def load(dataset="data/births.csv"):
    """"Transforms and Loads data into the local SQLite3 database"""

    #prints the full working directory and path
    print(os.getcwd())
    payload = csv.reader(open(dataset, newline=''), delimiter=',')
    next(payload)
    conn = sqlite3.connect('birthsDB.db')
    c = conn.cursor()
    c.execute("DROP TABLE IF EXISTS USBirths")
    c.execute("""
    CREATE TABLE USBirths (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        year INTEGER,
        month INTEGER,
        date_of_month INTEGER,
        day_of_week INTEGER,
        births INTEGER
    )
""")

    #insert
    c.executemany(
        """
        INSERT INTO USBirths (
            year, 
            month, 
            date_of_month, 
            day_of_week, 
            births
            ) 
            VALUES (?, ?, ?, ?, ?)""",
        payload,
    )


    conn.commit()
    conn.close()
    return "birthsDB.db"