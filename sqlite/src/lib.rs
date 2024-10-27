use csv::ReaderBuilder; // For loading from CSV
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File; // For loading CSV and handling errors

pub fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
    // Drop the existing table first (if it exists)
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);

    // Create the new table with the correct schema
    let create_query = format!(
        "CREATE TABLE {} (
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

// Read
pub fn query_exec(conn: &Connection, query_string: &str) -> Result<()> {
    let mut stmt = conn.prepare(query_string)?;

    let rows = stmt.query_map([], |row| {
        let year: i32 = row.get(0)?;
        let month: i32 = row.get(1)?;
        let date_of_month: i32 = row.get(2)?;
        let day_of_week: i32 = row.get(3)?;
        let births: i32 = row.get(4)?;
        Ok((year, month, date_of_month, day_of_week, births))
    })?;

    for row in rows {
        let (year, month, date_of_month, day_of_week, births) = row?;
        println!(
            "Year: {}, Month: {}, Date of Month: {}, Day of Week: {}, Births: {}",
            year, month, date_of_month, day_of_week, births
        );
    }

    Ok(())
}

// Delete (drop table)
pub fn drop_table(conn: &Connection, table_name: &str) -> Result<()> {
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);
    Ok(())
}

// Load data from a CSV file into the table
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

        conn.execute(
            &insert_query,
            params![year, month, date_of_month, day_of_week, births],
        )?;
    }

    println!(
        "Data loaded successfully from '{}' into table '{}'.",
        file_path, table_name
    );
    Ok(())
}
