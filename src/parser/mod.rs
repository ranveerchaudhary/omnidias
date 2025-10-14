pub mod tokenizer;
pub mod treebuilder;
pub mod executor;

#[derive(Debug)]
enum EitherOrType{
    Vec<InsertExpression>,
    Vec<SelectExpression>
}

#[derive(Debug)]
pub struct Query<'a> {
    pub query_type: &'a str,
    pub table: String,
    pub columns: Vec<String>,
    pub expression: EitherOrType
}

#[derive(Debug)]
pub struct Condition {
    pub object: String,
    pub operator: Operator,
    pub value: String,
}

#[derive(Debug)]
pub struct SelectExpression{
    pub condition: Condition,
    pub logical: Logical
}

#[derive(Debug)]
pub struct InsertExpression{
    pub values: Vec<String>
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