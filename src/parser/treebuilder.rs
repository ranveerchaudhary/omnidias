
pub struct query {
    query_type: String,
    table: String,
    columns: Vec<String>,
    condition: Vec<Condition>
}

pub struct Condition {
    column: String,
    operator: Operator,
    value: value
}

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

pub fn treebuilder(tokens: &Vec<&str>){

    println!("{:?}", tokens);

}