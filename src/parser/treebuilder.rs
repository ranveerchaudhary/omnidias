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

    let mut condition_object = Some(Condition {column: String::from("emp_id"), operator: Operator::Eq, value: String::from("1")});

    let mut query_object = Some(Query {query_type: String::from(&tokens[0]), table: String::from("VECokok"), columns: Vec::from([String::from("date_of_joining")]), condition: condition_object});

    println!("{:#?}", query_object);

}