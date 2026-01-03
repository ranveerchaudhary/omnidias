use super::Query;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn execute(parsed_query: &Option<Query>) {
    match parsed_query {
        Some(query) => {
            match query.query_type.as_str() {
                "select" => println!("This is a select query"),
                "insert" => println!("This is an insert query"),
                _ => {}
            }
        }
        None => {
            println!("No query to execute");
            let _ = executor("i", parsed_query);
        }
    }
}

fn executor(_type: &str, _parsed_query: &Option<Query>) -> std::io::Result<()> {
    let row = vec!["id1", "name", "age"];
    let storage_dir = "./stow.omda";

    let file = File::create(storage_dir)?;
    let mut f = BufWriter::new(file);
    f.write_all(&(row.len() as u32).to_le_bytes())?;

    for s in &row {
        let bytes = s.as_bytes();
        f.write_all(&(bytes.len() as u32).to_le_bytes())?;
        f.write_all(bytes)?;
    }

    println!("Saved");
    Ok(())
}