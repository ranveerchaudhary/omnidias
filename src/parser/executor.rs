use super::Query;

pub fn execute(parsed_query: &Option<Query>){
    match parsed_query {
        Some(query) => {
            if query.query_type == "select"{
                println!("This is a select query");
                // logic for executing the parsed query struct!
            }
        },
        None => {
            println!("No query to execute");
        }
    }
}