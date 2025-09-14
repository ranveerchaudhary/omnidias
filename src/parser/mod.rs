pub mod tokenizer;
pub mod treebuilder;
pub mod executor;

#[derive(Debug)]
pub struct Query<'a> {
    pub query_type: &'a str,
    pub table: String,
    pub columns: Vec<String>,
    pub expression: Vec<Expression>
}

#[derive(Debug)]
pub struct Condition {
    pub object: String,
    pub operator: Operator,
    pub value: String,
}

#[derive(Debug)]
pub struct Expression{
    pub condition: Condition,
    pub logical: Logical
}

#[derive(Debug)]
pub enum Logical {
    And,
    Or,
    End
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
    Lk,
    Error
}