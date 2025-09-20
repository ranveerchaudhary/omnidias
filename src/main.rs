use std::time::Instant;
mod parser;


fn main() {
    // Handle potential error
    //let query = "select * from vectable where vector ~ [[0.23, 0.4, 0.1]]";
    let query = "insert into vectable columns(*) values(chunks, [[0.02, 0.15, 0.74]]);";
    let percont = Instant::now();
    let tokenized_query = parser::tokenizer::tokenizer(&query.replace(";", ""));
    let parsed_query = parser::treebuilder::tree_builder(&tokenized_query, String::from(query));
    let time_be = percont.elapsed();
    println!("{:?}", &time_be);
    println!("{:?}", parsed_query);
    let execute_query = parser::executor::execute(&parsed_query);
    println!("{:?}", execute_query);
    //println!("{:?}", parsed_query);

    // after running simualtions 
    //    min = 411.000 µs
    // max 959.000 µs
    // mean = 747.790 µs
}
