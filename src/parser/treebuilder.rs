#[derive(Debug)]
pub struct Query<'a> {
    query_type: &'a str,
    table: String,
    columns: Vec<String>,
    expression: Vec<Expression>
}

#[derive(Debug)]
pub struct Condition {
    object: String,
    operator: Operator,
    value: String,
}

#[derive(Debug)]
pub struct Expression{
    condition: Condition,
    logical: Logical
}

#[derive(Debug)]
pub enum Logical {
    and,
    or,
    end
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
    Lk
}

//so the operators defined here (custom) goes as follows
//eq -> equals to (=) ; lt -> less than (<) ; gt -> greater than (>)
// ne -> not equivalent to (!= or <>) ; le -> less than equals to (<=) ; greater than equals to (>=);
// cl -> closet to (~) [this will be used for vectors sorta like ANN]
// ft -> farthest to (!~) [again for vectors ANN]

fn get_operator(operator_sign: &str) -> Operator{
    
    match operator_sign{
        "=" => Operator::Eq,
        "<" => Operator::Lt,
        ">" => Operator::Gt,
        "<>" | "!="=> Operator::Ne,
        "<=" => Operator::Le,
        ">=" => Operator::Ge,
        "~" => Operator::Cl,
        "~!" => Operator::Ft,
        "like" => Operator::Lk,
        _ => return panic!("Unknown operator!!"),

    }
}

pub fn tree_builder(tokens: &[String]) -> Option<Query>{

    //println!("{:?}", tokens);

    let mut pointer: usize = 0;
    let query_type_value = &tokens[pointer];

    let mut column_list: Vec<String> = Vec::with_capacity(500);

    let mut table: String = String::new();

    let mut query_object: Option<Query> = None;

    if query_type_value == "select"{

        pointer += 1;
        while pointer < tokens.len() && tokens[pointer] != "from" {
            column_list.push(tokens[pointer].trim_matches(',').to_string());
            pointer += 1;
        }
        
        pointer += 1;
        table = String::from(tokens[pointer].clone());

        let mut all_expressions = Vec::new();

        pointer += 1;
        if tokens[pointer] == "where"{
            pointer += 1;

            while pointer <= tokens.len()-2{
                let condition = Condition{object: tokens[pointer].clone(), operator: get_operator(&tokens[pointer+1]), value: tokens[pointer+2].clone()};
                
                let current_logical_operator: Logical = Logical::end;

                pointer += 3;

                if pointer < tokens.len(){
                    
                    if tokens[pointer] == "and"{
                        let current_logical_operator: Logical = Logical::and;
                    }
                    else{
                        let current_logical_operator: Logical = Logical::or;
                    }

                    pointer += 1;
                }

                let expression: Expression = Expression{condition: condition, logical: current_logical_operator};

                all_expressions.push(expression);
            }

        }

        query_object = Some(Query {query_type: query_type_value, table: table, columns: column_list, expression: all_expressions});
    }
    
    return query_object
    //println!("{:#?}", query_object);

}