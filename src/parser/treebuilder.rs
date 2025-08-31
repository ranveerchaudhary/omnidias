#[derive(Debug)]
pub struct Query {
    query_type: String,
    table: String,
    columns: Vec<String>,
    condition: Option<Condition>
}

#[derive(Debug)]
pub struct Condition {
    column: String,
    operator: Operator,
    value: String
}

#[derive(Debug)]
pub enum Operator {
    Eq,
    Lt,
    Gt,
    Ne,
    Le,
    Ge,
    Cl,
    Ft,
}

//so the operators defined here (custom) goes as follows
//eq -> equals to (=) ; lt -> less than (<) ; gt -> greater than (>)
// ne -> not equivalent to (!=) ; le -> less than equals to (<=) ; greater than equals to (>=);
// cl -> closet to (~) [this will be used for vectors sorta like ANN]
// ft -> farthest to (!~) [again for vectors ANN]

pub fn treebuilder(tokens: &Vec<String>){

    println!("{:?}", tokens);

    let mut pointer: usize = 0;
    let query_type_value = String::from(tokens[pointer].clone());

    let mut column_list: Vec<String> = Vec::new();

    let mut table = String::new();

    if &query_type_value == "select"{
        pointer += 1;
        while pointer < tokens.len() && tokens[pointer] != "from" {
            column_list.push(tokens[pointer].trim_matches(',').clone().to_string());
            pointer += 1;
        }
        
        pointer += 1;
        table = String::from(tokens[pointer].clone());

        pointer += 1;
        if tokens[pointer] == "where"{
            pointer += 1

        }

    }
    

    let mut condition_object = Some(Condition {column: String::from("emp_id"), operator: Operator::Eq, value: String::from("1")});

    let mut query_object = Some(Query {query_type: query_type_value, table: table, columns: column_list, condition: condition_object});

    println!("{:#?}", query_object);

}