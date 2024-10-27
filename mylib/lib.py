import sqlite3
import csv

# Create table function
def create_table(conn, table_name):
    try:
        # Drop the table if it exists
        drop_query = f"DROP TABLE IF EXISTS {table_name}"
        conn.execute(drop_query)
        print(f"Table '{table_name}' dropped successfully.")

        # Create the new table with the given schema
        create_query = f"""
        CREATE TABLE {table_name} (
            year INTEGER NOT NULL,
            month INTEGER NOT NULL,
            date_of_month INTEGER NOT NULL,
            day_of_week INTEGER NOT NULL,
            births INTEGER NOT NULL
        )
        """
        conn.execute(create_query)
        print(f"Table '{table_name}' created successfully.")
    except sqlite3.Error as e:
        print(f"Error creating table: {e}")

# Read data function
def query_exec(conn, query_string):
    try:
        cursor = conn.execute(query_string)
        rows = cursor.fetchall()
        for row in rows:
            year, month, date_of_month, day_of_week, births = row
            print(f"Year: {year}, Month: {month}, Date of Month: {date_of_month}, Day of Week: {day_of_week}, Births: {births}")
    except sqlite3.Error as e:
        print(f"Error executing query: {e}")

# Drop table function
def drop_table(conn, table_name):
    try:
        drop_query = f"DROP TABLE IF EXISTS {table_name}"
        conn.execute(drop_query)
        print(f"Table '{table_name}' dropped successfully.")
    except sqlite3.Error as e:
        print(f"Error dropping table: {e}")

# Load data from CSV function
def load_data_from_csv(conn, table_name, file_path):
    try:
        with open(file_path, newline='') as csvfile:
            reader = csv.reader(csvfile)
            next(reader, None)  # Skip header if it exists

            insert_query = f"""
            INSERT INTO {table_name} (year, month, date_of_month, day_of_week, births)
            VALUES (?, ?, ?, ?, ?)
            """
            for record in reader:
                year, month, date_of_month, day_of_week, births = map(int, record)
                conn.execute(insert_query, (year, month, date_of_month, day_of_week, births))
            conn.commit()
        print(f"Data loaded successfully from '{file_path}' into table '{table_name}'.")
    except Exception as e:
        print(f"Error loading data from CSV: {e}")

# Usage example
if __name__ == "__main__":
    # Connect to the database (creates it if it doesn't exist)
    conn = sqlite3.connect('births_data.db')
    
    # Define table name and file path
    table_name = "births"
    file_path = "births.csv"  # Replace with the actual CSV file path
    
    # Create the table
    create_table(conn, table_name)
    
    # Load data from CSV
    load_data_from_csv(conn, table_name, file_path)
    
    # Query data
    query_exec(conn, f"SELECT * FROM {table_name}")
    
    # Drop the table (if needed)
    drop_table(conn, table_name)
    
    # Close the connection
    conn.close()