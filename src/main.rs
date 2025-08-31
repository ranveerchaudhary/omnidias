use std::time::Instant;
use std::io;
use std::fs;
mod parser;


fn main() {
    // Handle potential error
    let query = String::from("SELECT VECTORS, chunks, links FROM vectable WHERE embeddings CLOSEST [[0.3345343, 0.435454645, 0.45465656]] AND METHOD IS HSNW");
    let tokenizedquery = parser::tokenizer::tokenizer(&query);
    parser::treebuilder::treebuilder(&tokenizedquery);
}
