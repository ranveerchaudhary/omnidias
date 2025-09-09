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

//so the operators defined here (custom) goes as follows
//eq -> equals to (=) ; lt -> less than (<) ; gt -> greater than (>)
// ne -> not equivalent to (!= or <>) ; le -> less than equals to (<=) ; greater than equals to (>=);
// cl -> closet to (~) [this will be used for vectors sorta like ANN]
// ft -> farthest to (!~) [again for vectors ANN]

fn is_table(table_name: &str) -> bool{

    if table_name == "vectable"{
        return true;
    }
    else{
        return false;
    }

    // have some logic to validate table 

}

fn error_printer(at_position: usize, original_query: &str, tokens: &[String], error_message: &str){

    let first_part = tokens[0..at_position].join(" ");
    let middle_part = &tokens[at_position];
    let last_part = tokens[at_position + 1..].join(" ");

    println!(
        "{} {} {}",
        first_part,
        format!("\x1B[4m{}\x1B[0m", middle_part),
        last_part
    );


    panic!("{}", error_message);
}

fn get_operator(operator_sign: &str, at_position: usize, original_query: &str, tokens: &[String]) -> Operator{
    
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
        _ => {
            error_printer(at_position, original_query, tokens, "Unknown operator!");
            return Operator::Error
        },
    }
}

pub fn tree_builder(tokens: &[String], original_query: String) -> Option<Query>{

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

            if pointer >= tokens.len()-2 && tokens[pointer] != "from"{

                println!("{}", original_query);
                panic!("No `FROM` clause in query!");
            }
        }

        pointer += 1;
        table = String::from(tokens[pointer].clone());

        if is_table(&table){
            // it saul goodman
        }
        else{
            error_printer(pointer, &original_query, tokens, "Table doesn't exist!");
        }

        let mut all_expressions = Vec::new();

        pointer += 1;

        if pointer < tokens.len()-1{

            if tokens[pointer] == "where"{

                pointer += 1;

                while pointer <= tokens.len()-2{
                    let condition = Condition{object: tokens[pointer].clone(), operator: get_operator(&tokens[pointer+1], pointer+1, &original_query, tokens), value: tokens[pointer+2].clone()};

                    let mut current_logical_operator: Logical = Logical::End;

                    pointer += 3;

                    if pointer < tokens.len(){
                        
                        if tokens[pointer] == "and"{
                            current_logical_operator = Logical::And;
                        }

                        else if tokens[pointer] == "or"{
                            current_logical_operator = Logical::Or;
                        }

                        else{
                            error_printer(pointer, &original_query, tokens, "Unknown Logical Operator! (Did you mean `and/or`?)");
                        }

                        pointer += 1;
                    }

                    let expression: Expression = Expression{condition: condition, logical: current_logical_operator};

                    all_expressions.push(expression);
                }

            }

            else{
            
                error_printer(pointer, &original_query, tokens, "Unknown Keyword! (Did you mean `Where`?)");

            }
        
        }

        query_object = Some(Query {query_type: query_type_value, table: table, columns: column_list, expression: all_expressions});
    }
    
    return query_object
    //println!("{:#?}", query_object);

}