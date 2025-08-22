use std::time::Instant;
use std::io;
use std::fs;
mod query_parser;

fn store_vectors() -> io::Result<()> {
    let vectors: [f64; 3] = [0.45, 0.355532, 0.2456];

    // Convert array to a comma-separated string
    let content = vectors.iter()
                         .map(|v| v.to_string())
                         .collect::<Vec<String>>()
                         .join(", ");

    // Write to file, propagate error if any
    fs::write("omnidias.omda", content)?;

    // Print the vectors
    println!("These are the vectors: {:?}", vectors);

    Ok(())
}

fn parse_query(query: &str) -> String{
    return "".to_string();
}

fn main() {
    // Handle potential error
    let query = String::from("SELECT VECTORS FROM VECTABLE WHERE CLOSEST [[0.3345343, 0.435454645, 0.45465656]] AND METHOD IS HSNW");
    println!("{:#?}", query_parser::parse_query(&query));
}
