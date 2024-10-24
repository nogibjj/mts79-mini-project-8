// This will be the CLI portion of the project where we accept
// user-defined arguments and call lib.rs logic to handle them
use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use sqlite::{create_table, drop_table, load_data_from_csv, query_exec}; // import library logic

// Define a struct to hold our CLI arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Enum for commands: Create, Query (Read/Update), Delete, Load
#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a table by passing a table name
    #[command(alias = "c", short_flag = 'c')]
    Create { table_name: String },

    /// Execute a query by passing a query string (Read or Update operations)
    #[command(alias = "q", short_flag = 'q')]
    Query { query: String },

    /// Drop a table by passing a table name
    #[command(alias = "d", short_flag = 'd')]
    Delete { table_name: String },

    /// Load data from a CSV file by passing a table name and file path
    #[command(alias = "l", short_flag = 'l')]
    Load {
        table_name: String,
        file_path: String,
    },
}

fn main() -> Result<()> {
    // Parse the CLI arguments
    let args = Cli::parse();

    // Establish a connection to the SQLite database
    let conn = Connection::open("births_database.db")?;

    // Match the subcommand and execute the corresponding library function
    match args.command {
        Commands::Create { table_name } => {
            println!("Creating Table '{}'", table_name);
            create_table(&conn, &table_name).expect("Failed to create table");
        }
        Commands::Query { query } => {
            println!("Executing Query: '{}'", query);
            query_exec(&conn, &query).expect("Failed to execute query");
        }
        Commands::Delete { table_name } => {
            println!("Dropping Table '{}'", table_name);
            drop_table(&conn, &table_name).expect("Failed to drop table");
        }
        Commands::Load {
            table_name,
            file_path,
        } => {
            println!(
                "Loading data into table '{}' from '{}'",
                table_name, file_path
            );
            load_data_from_csv(&conn, &table_name, &file_path)
                .expect("Failed to load data from CSV");
        }
    }

    Ok(())
}
