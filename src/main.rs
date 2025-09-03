use std::time::Instant;
use std::io;
use std::fs;
mod parser;


fn main() {
    // Handle potential error
    let percont = Instant::now();
    let query = String::from("SELECT VECTORS, chunks, links FROM vectable WHERE embeddings ~ [[0.3345343, 0.435454645, 0.45465656]] AND METHOD = HSNW");
    let tokenized_query = parser::tokenizer::tokenizer(&query);
    parser::treebuilder::tree_builder(&tokenized_query);

    println!("{:?}", percont.elapsed());
}
