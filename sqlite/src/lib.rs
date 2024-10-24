// Import necessary crates for CSV handling and SQLite database interaction
use csv::ReaderBuilder;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File;

// Create a table
pub fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
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
    let rows = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let year: i32 = row.get(1)?;
        let month: i32 = row.get(2)?;
        let date_of_month: i32 = row.get(3)?;
        let day_of_week: i32 = row.get(4)?;
        let births: i32 = row.get(5)?;
        Ok((id, year, month, date_of_month, day_of_week, births))
    })?;

    for row in rows {
        let (id, year, month, date_of_month, day_of_week, births) = row?;
        println!("ID: {}, Year: {}, Month: {}, Date: {}, Day: {}, Births: {}", id, year, month, date_of_month, day_of_week, births);
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

    for result in rdr.records() {
        let record = result?;
        let year: i32 = record[0].parse()?;
        let month: i32 = record[1].parse()?;
        let date_of_month: i32 = record[2].parse()?;
        let day_of_week: i32 = record[3].parse()?;
        let births: i32 = record[4].parse()?;

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