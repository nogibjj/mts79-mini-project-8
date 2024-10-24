use csv::ReaderBuilder; // for loading from csv
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File; // for loading csv and handling errors

// Create a table with the births dataset schema
pub fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            year INTEGER NOT NULL,
            month INTEGER NOT NULL,
            date_of_month INTEGER NOT NULL,
            day_of_week TEXT NOT NULL,
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
        let year: i64 = row.get(0)?;
        let month: i64 = row.get(1)?;
        let date_of_month: i64 = row.get(2)?;
        let day_of_week: String = row.get(3)?;
        let births: i64 = row.get(4)?;
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

// Drop the table
pub fn drop_table(conn: &Connection, table_name: &str) -> Result<()> {
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);
    Ok(())
}

// Load data from a CSV file into the births table
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

    // Loop that expects the specific schema for births
    for result in rdr.records() {
        let record = result?;
        let year: i64 = record[0].parse().expect("Failed to parse year");
        let month: i64 = record[1].parse().expect("Failed to parse month");
        let date_of_month: i64 = record[2].parse().expect("Failed to parse date_of_month");
        let day_of_week: &str = &record[3];
        let births: i64 = record[4].parse().expect("Failed to parse births");

        conn.execute(&insert_query, params![year, month, date_of_month, day_of_week, births])?;
    }

    println!(
        "Data loaded successfully from '{}' into table '{}'.",
        file_path, table_name
    );
    Ok(())
}

// Update records in the births table
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