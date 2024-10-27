"""
Query the database
"""

import sqlite3

# Define a global variable for the log file
LOG_FILE = "query_log.md"


def log_query(query):
    """Adds to a query markdown file"""
    with open(LOG_FILE, "a") as file:
        file.write(f"```sql\n{query}\n```\n\n")


def general_query(query):
    """Runs a query a user inputs"""
    # Connect to the SQLite database
    conn = sqlite3.connect("birthsDB.db")

    # Create a cursor object to execute SQL queries
    cursor = conn.cursor()

    # Execute the query
    cursor.execute(query)

    # If the query modifies the database, commit the changes
    if (
        query.strip().lower().startswith("insert")
        or query.strip().lower().startswith("update")
        or query.strip().lower().startswith("delete")
    ):
        conn.commit()

    # Close the cursor and connection
    cursor.close()
    conn.close()

    log_query(f"{query}")


def create_record(year, month, date_of_month, day_of_week, births):
    """Create a new record"""
    conn = sqlite3.connect("birthsDB.db")
    c = conn.cursor()
    c.execute(
        """
        INSERT INTO USBirths 
        (year, month, date_of_month, day_of_week, births) 
        VALUES (?, ?, ?, ?, ?)
        """,
        (year, month, date_of_month, day_of_week, births),
    )
    conn.commit()
    conn.close()

    # Log the query
    log_query(
        f"""INSERT INTO USBirths VALUES (
            {year}, 
            {month}, 
            {date_of_month}, 
            {day_of_week}, 
            {births});"""
    )


def update_record(record_id, year, month, date_of_month, day_of_week, births):
    """Update an existing record"""
    conn = sqlite3.connect("birthsDB.db")
    c = conn.cursor()
    c.execute(
        """
        UPDATE USBirths 
        SET year=?, 
            month=?, 
            date_of_month=?, 
            day_of_week=?, 
            births=? 
        WHERE rowid=?
        """,
        (year, month, date_of_month, day_of_week, births, record_id),
    )
    conn.commit()
    conn.close()

    # Log the query
    log_query(
        f"""UPDATE USBirths SET 
        year={year}, 
        month={month}, 
        date_of_month={date_of_month}, 
        day_of_week={day_of_week}, 
        births={births} 
        WHERE rowid={record_id};"""
    )


def delete_record(record_id):
    """Delete a record"""
    conn = sqlite3.connect("birthsDB.db")
    c = conn.cursor()
    c.execute("DELETE FROM USBirths WHERE rowid=?", (record_id,))
    conn.commit()
    conn.close()

    # Log the query
    log_query(f"DELETE FROM USBirths WHERE rowid={record_id};")


def read_data():
    """Read the first 10 rows of the data"""
    conn = sqlite3.connect("birthsDB.db")
    c = conn.cursor()
    c.execute("SELECT * FROM USBirths LIMIT 10")
    data = c.fetchall()
    log_query("SELECT * FROM USBirths LIMIT 10;")
    conn.close()
    return data