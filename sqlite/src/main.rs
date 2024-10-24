use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use sqlite::{create_table, drop_table, load_data_from_csv, query_exec, update_table}; // import library logic

// This struct defines the CLI arguments for the application
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Enum for handling different commands: Create, Query, Delete, Load, and Update
#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a table with the specified name
    #[command(alias = "c", short_flag = 'c')]
    Create { table_name: String },
    /// Execute a query (Read or Update operations)
    #[command(alias = "q", short_flag = 'q')]
    Query { query: String },
    /// Drop a table by name
    #[command(alias = "d", short_flag = 'd')]
    Delete { table_name: String },
    /// Load data from a CSV file into a table
    #[command(alias = "l", short_flag = 'l')]
    Load {
        table_name: String,
        file_path: String,
    },
    /// Update records in the table using set clause and condition
    #[command(alias = "u", short_flag = 'u')]
    Update {
        table_name: String,
        set_clause: String,
        condition: String,
    },
}

fn main() -> Result<()> {
    // Parse the CLI arguments
    let args = Cli::parse();
    // Open a connection to the database (create or use existing)
    let conn = Connection::open("births_database.db")?;

    // Match the subcommand and execute the appropriate logic
    match args.command {
        Commands::Create { table_name } => {
            println!("Creating table '{}'", table_name);
            create_table(&conn, &table_name).expect("Failed to create table");
        }
        Commands::Query { query } => {
            println!("Executing query: '{}'", query);
            query_exec(&conn, &query).expect("Failed to execute query");
        }
        Commands::Delete { table_name } => {
            println!("Dropping table '{}'", table_name);
            drop_table(&conn, &table_name).expect("Failed to drop table");
        }
        Commands::Load {
            table_name,
            file_path,
        } => {
            println!("Loading data into '{}' from '{}'", table_name, file_path);
            load_data_from_csv(&conn, &table_name, &file_path)
                .expect("Failed to load data from CSV");
        }
        Commands::Update {
            table_name,
            set_clause,
            condition,
        } => {
            println!("Updating table '{}'", table_name);
            update_table(&conn, &table_name, &set_clause, &condition)
                .expect("Failed to update table");
        }
    }

    Ok(())
}