use std::time::Instant;
use std::io;
use std::fs;
mod parser;


fn main() {
    // Handle potential error
    let query = "SELECT VECTORS, chunks, links FROM vectable WHERE embeddings ~ [[0.3345343, 0.435454645, 0.45465656]] AND METHOD = HSNW";
    let percont = Instant::now();
    let tokenized_query = parser::tokenizer::tokenizer(&query);
    let parsed_query = parser::treebuilder::tree_builder(&tokenized_query);
    let time_be = percont.elapsed();
    println!("{:?}", &time_be);
    //println!("{:?}", parsed_query);

    // after running simualtions 
    //    min = 411.000 µs
    // max 959.000 µs
    // mean = 747.790 µs
}
