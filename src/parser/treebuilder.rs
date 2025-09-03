#[derive(Debug)]
pub struct Query<'a> {
    query_type: &'a str,
    table: String,
    columns: Vec<String>,
    condition: Vec<Condition>
}

#[derive(Debug)]
pub struct Condition {
    object: String,
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

    let mut column_list: Vec<String> = Vec::with_capacity(500);;

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

        let mut all_conditions = Vec::new();

        pointer += 1;
        if tokens[pointer] == "where"{
            pointer += 1;

            while pointer <= tokens.len()-2{
                let condition = Condition{object: tokens[pointer].clone(), operator: get_operator(&tokens[pointer+1]), value: tokens[pointer+2].clone()};
                all_conditions.push(condition);
                pointer += 3;

                if pointer < tokens.len(){
                    // add logic for and / or
                    pointer += 1
                }
            }

        }

        query_object = Some(Query {query_type: query_type_value, table: table, columns: column_list, condition: all_conditions});
    }
    
    return query_object
    //println!("{:#?}", query_object);

}