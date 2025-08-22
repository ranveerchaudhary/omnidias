fn spliter(query: &str) -> Vec<String> {

    if query.is_empty() {
        println!("query is empty!");
        return Vec::new();

    } 
    
    else {
        let mut elements = Vec::new();

        for element in query.split_whitespace() {
            elements.push(String::from(element).to_lowercase());
        }
        return elements;
    }
}

fn executor(parsed: &mut Vec<String>, original: &str) {

    if parsed.is_empty() {
        println!("Error: No command provided.");
        return;
    }

    let operation = &parsed[0];

    if operation == "select" {
        println!("select operation!");

        let table_index = parsed.iter().position(|x| x == "from").unwrap() + 1;
        let table = &parsed.get(table_index).unwrap();

        let vector_index_start = parsed.iter().position(|x| x == "closet").unwrap() + 1;
        let vector_index_end = parsed.iter().position(|x| x == "and").unwrap();
        let vector = &parsed.get(table_index).unwrap();

        println!("Table is {}", &table);
        println!("Vectors are -> {}", )

    }
    
    else if operation == "insert" {
        println!("insert operation!");

    } 
    
    else if operation == "delete" {
        println!("delete operation!");
        
    } 

    else {

        let unknown_ops = parsed.remove(0);
        
        println!("<{}> {}...", unknown_ops.to_uppercase(), parsed[..4].join(" "));
        
        let spaces = (String::from(" ")).repeat(unknown_ops.chars().count() / 2);
        println!("{}{}", spaces, "^");
        
        println!("Error parsing query!");
    }
}

// fn view_query() {}
// fn modify_query() {}
// fn remove_query() {}

pub fn parse_query(query: &str) {
    let mut parsed_query = spliter(query);
    executor(&mut parsed_query, query);
}

// SELECT -> VIEW
// INSERT -> SINGLE OR MULTIPLE
// DELECT -> WHERE EQUALS OR CLOSET TOP N