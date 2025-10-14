use super::Query;
use std::fs;

pub fn execute(parsed_query: &Option<Query>){


    println!("{:?} jhbj", &parsed_query);

    match parsed_query {


        Some(query) => {
            if query.query_type == "select"{
                println!("This is a select query");
                // logic for executing the parsed query struct!
            }
            if query.query_type == "insert"{
                println!("This is a insert query!");
            }
        },
        None => {
            println!("No query to execute");
            executor("i", &parsed_query);
        }
    }
}

fn executor(type_: &str, parsed_query: &Option<Query>) -> std::io::Result<()>{
    let row = vec!["id1", "name", "age"];

    let storage_dir = "./stow.omda"; // for experimental dev work lets have this

    use std::io::Write;
    let mut f = std::fs::File::create(storage_dir)?;
    // write count then each string as: len(u32 LE) + bytes
    f.write_all(&(row.len() as u32).to_le_bytes())?;
    for s in &row {
        let b = s.as_bytes();
        f.write_all(&(b.len() as u32).to_le_bytes())?;
        f.write_all(b)?;
    }

    println!("Saved!!");
    Ok(())
}