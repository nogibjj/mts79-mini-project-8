import argparse
import sqlite3
from lib import create_table, drop_table, load_data_from_csv, query_exec

def main():
    # Set up command-line argument parsing
    parser = argparse.ArgumentParser(description="CLI for database operations on birth data")
    subparsers = parser.add_subparsers(dest="command", help="Available commands")

    # Create table command
    parser_create = subparsers.add_parser("create", aliases=["c"], help="Create a new table")
    parser_create.add_argument("table_name", type=str, help="Name of the table to create")

    # Query command
    parser_query = subparsers.add_parser("query", aliases=["q"], help="Execute a query")
    parser_query.add_argument("query", type=str, help="SQL query to execute (Read/Update)")

    # Delete (drop table) command
    parser_delete = subparsers.add_parser("delete", aliases=["d"], help="Drop a table")
    parser_delete.add_argument("table_name", type=str, help="Name of the table to drop")

    # Load data from CSV command
    parser_load = subparsers.add_parser("load", aliases=["l"], help="Load data from a CSV file")
    parser_load.add_argument("table_name", type=str, help="Name of the table to load data into")
    parser_load.add_argument("file_path", type=str, help="Path to the CSV file")

    # Parse the arguments
    args = parser.parse_args()

    # Establish a connection to the SQLite database
    conn = sqlite3.connect("births_databasepy.db")

    # Execute the relevant function based on the command
    if args.command in ["create", "c"]:
        print(f"Creating Table '{args.table_name}'")
        create_table(conn, args.table_name)
    elif args.command in ["query", "q"]:
        print(f"Executing Query: '{args.query}'")
        query_exec(conn, args.query)
    elif args.command in ["delete", "d"]:
        print(f"Dropping Table '{args.table_name}'")
        drop_table(conn, args.table_name)
    elif args.command in ["load", "l"]:
        print(f"Loading data into table '{args.table_name}' from '{args.file_path}'")
        load_data_from_csv(conn, args.table_name, args.file_path)
    else:
        parser.print_help()

    # Close the database connection
    conn.close()

if __name__ == "__main__":
    main()
