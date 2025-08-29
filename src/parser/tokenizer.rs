use regex::Regex;

pub fn tokenizer(query: &str) -> Vec<String>{

    if query.is_empty() {
        println!("query is empty!");
        return Vec::new()

    } 
    
    else {
        let re = Regex::new(r"\[\[[^\]]*\]\]|\[[^\]]*\]|[^\s\[\]]+").unwrap();
        return re.find_iter(query).map(|m| m.as_str().to_lowercase()).collect();

    }
}