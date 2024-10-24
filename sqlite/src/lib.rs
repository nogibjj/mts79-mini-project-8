// Import necessary crates for CSV handling and SQLite database interaction
use csv::ReaderBuilder;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File;

// Function to create a table with the specified schema for births dataset
pub fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            year INTEGER NOT NULL,
            month INTEGER NOT NULL,
            date_of_month INTEGER NOT NULL,
            day_of_week INTEGER NOT NULL,
            births INTEGER NOT NULL
        )",
        table_name
    );
    conn.execute(&create_query, [])?;
    println!("Table '{}' created successfully.", table_name);
    Ok(())
}

// Read data from the table
pub fn query_exec(conn: &Connection, query_string: &str) -> Result<()> {
    let mut stmt = conn.prepare(query_string)?;

    // Map the rows to the columns in the births table
    let rows = stmt.query_map([], |row| {
        // Use named columns instead of index-based retrieval
        let year: i64 = row.get("year")?;
        let month: i64 = row.get("month")?;
        let date_of_month: i64 = row.get("date_of_month")?; // Fixed to correctly reference column
        let day_of_week: i64 = row.get("day_of_week")?;
        let births: i64 = row.get("births")?;
        Ok((year, month, date_of_month, day_of_week, births))
    })?;

    // Print the results
    for row in rows {
        let (year, month, date_of_month, day_of_week, births) = row?;
        println!(
            "Year: {}, Month: {}, Date: {}, Day of Week: {}, Births: {}",
            year, month, date_of_month, day_of_week, births
        );
    }

    Ok(())
}


// Function to drop a table by its name
pub fn drop_table(conn: &Connection, table_name: &str) -> Result<()> {
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);
    Ok(())
}

// Function to load data from a CSV file into the births table
pub fn load_data_from_csv(
    conn: &Connection,
    table_name: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let insert_query = format!(
        "INSERT INTO {} (year, month, date_of_month, day_of_week, births) VALUES (?, ?, ?, ?, ?)",
        table_name
    );

    // Loop through each record in the CSV file and insert it into the table
    for result in rdr.records() {
        let record = result?;
        let year: i64 = record[0].parse().expect("Failed to parse year");
        let month: i64 = record[1].parse().expect("Failed to parse month");
        let date_of_month: i64 = record[2].parse().expect("Failed to parse date_of_month");
        let day_of_week: i64 = record[3].parse().expect("Failed to parse day_of_week");
        let births: i64 = record[4].parse().expect("Failed to parse births");

        conn.execute(&insert_query, params![year, month, date_of_month, day_of_week, births])?;
    }

    println!(
        "Data loaded successfully from '{}' into table '{}'.",
        file_path, table_name
    );
    Ok(())
}

// Function to update records in the table using a set clause and condition
pub fn update_table(
    conn: &Connection,
    table_name: &str,
    set_clause: &str,
    condition: &str,
) -> Result<()> {
    let update_query = format!(
        "UPDATE {} SET {} WHERE {};",
        table_name, set_clause, condition
    );
    
    let affected_rows = conn.execute(&update_query, [])?;
    
    println!(
        "Successfully updated {} row(s) in table '{}'.",
        affected_rows, table_name
    );
    
    Ok(())
}