use std::{collections::HashMap, fs, process::Command, vec};

use crate::Token::{Token, TokensTypes};
// suma resta multiplicar division 1+1
/**
 * porfavor cuando se utilize esto no incluir el primer if
 * ejemplo:
 * if 3== 3
 * ...
 * end-if
 * ->
 * ...
 *
 * TODO: verificar si funciona o no hjrejhwhjhehjwhjhbhhhhhhhhhhhhhhhhhhhpppppppppppppppppppppppppppppppppppppppppppppppp
 */
pub fn get_if_tokens(tokens: Vec<Token>) -> Vec<Token> {
    let mut new_tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let mut others_if: usize = 0;
    let mut others_if_ends: usize = 0;
    let mut stop: bool = false;
    while i < tokens.len() {
        if stop == false {
            if tokens[i].get_type() == TokensTypes::IF_CONDITION {
                others_if += 1;
            } else if tokens[i].get_type() == TokensTypes::END_IF {
                if others_if != others_if_ends + 1 {
                    stop = true;
                } else {
                    others_if_ends += 1;
                }
            }
            if stop != true {
                new_tokens.push(tokens[i].clone());
            }
            i += 1;
        }
    }
    new_tokens
}
pub fn tranform_math_to_result(tokens_to_transform: Vec<Token>) -> Result<Token, String> {
    if tokens_to_transform.len() < 3 {
        return Err("invalid math".to_string());
    }

    if tokens_to_transform[0].get_type() == TokensTypes::INT
        && tokens_to_transform[1].get_type() == TokensTypes::PLUS
        && tokens_to_transform[2].get_type() == TokensTypes::INT
    {
        return Ok(Token::new(
            format!(
                "{}",
                tokens_to_transform[0]
                    .get_value()
                    .parse::<isize>()
                    .ok()
                    .unwrap()
                    + tokens_to_transform[2]
                        .get_value()
                        .parse::<isize>()
                        .ok()
                        .unwrap()
            ),
            TokensTypes::INT,
        ));
    }
    if tokens_to_transform[0].get_type() == TokensTypes::INT
        && tokens_to_transform[1].get_type() == TokensTypes::MINUS
        && tokens_to_transform[2].get_type() == TokensTypes::INT
    {
        return Ok(Token::new(
            format!(
                "{}",
                tokens_to_transform[0]
                    .get_value()
                    .parse::<isize>()
                    .ok()
                    .unwrap()
                    - tokens_to_transform[2]
                        .get_value()
                        .parse::<isize>()
                        .ok()
                        .unwrap()
            ),
            TokensTypes::INT,
        ));
    }
    if tokens_to_transform[0].get_type() == TokensTypes::INT
        && tokens_to_transform[1].get_type() == TokensTypes::MULT
        && tokens_to_transform[2].get_type() == TokensTypes::INT
    {
        return Ok(Token::new(
            format!(
                "{}",
                tokens_to_transform[0]
                    .get_value()
                    .parse::<isize>()
                    .ok()
                    .unwrap()
                    * tokens_to_transform[2]
                        .get_value()
                        .parse::<isize>()
                        .ok()
                        .unwrap()
            ),
            TokensTypes::INT,
        ));
    }
    if tokens_to_transform[0].get_type() == TokensTypes::INT
        && tokens_to_transform[1].get_type() == TokensTypes::DIV
        && tokens_to_transform[2].get_type() == TokensTypes::INT
    {
        return Ok(Token::new(
            format!(
                "{}",
                tokens_to_transform[0]
                    .get_value()
                    .parse::<isize>()
                    .ok()
                    .unwrap()
                    / tokens_to_transform[2]
                        .get_value()
                        .parse::<isize>()
                        .ok()
                        .unwrap()
            ),
            TokensTypes::INT,
        ));
    }

    if tokens_to_transform[0].get_type() == TokensTypes::FLOAT
        && tokens_to_transform[1].get_type() == TokensTypes::PLUS
        && tokens_to_transform[2].get_type() == TokensTypes::FLOAT
    {
        return Ok(Token::new(
            format!(
                "{}",
                tokens_to_transform[0].get_value().parse::<f64>().unwrap()
                    + tokens_to_transform[2]
                        .get_value()
                        .parse::<f64>()
                        .ok()
                        .unwrap()
            ),
            TokensTypes::FLOAT,
        ));
    }
    if tokens_to_transform[0].get_type() == TokensTypes::FLOAT
        && tokens_to_transform[1].get_type() == TokensTypes::MINUS
        && tokens_to_transform[2].get_type() == TokensTypes::FLOAT
    {
        return Ok(Token::new(
            format!(
                "{}",
                tokens_to_transform[0].get_value().parse::<f64>().unwrap()
                    - tokens_to_transform[2]
                        .get_value()
                        .parse::<f64>()
                        .ok()
                        .unwrap()
            ),
            TokensTypes::FLOAT,
        ));
    }
    if tokens_to_transform[0].get_type() == TokensTypes::FLOAT
        && tokens_to_transform[1].get_type() == TokensTypes::MULT
        && tokens_to_transform[2].get_type() == TokensTypes::FLOAT
    {
        return Ok(Token::new(
            format!(
                "{}",
                tokens_to_transform[0].get_value().parse::<f64>().unwrap()
                    * tokens_to_transform[2]
                        .get_value()
                        .parse::<f64>()
                        .ok()
                        .unwrap()
            ),
            TokensTypes::FLOAT,
        ));
    }
    if tokens_to_transform[0].get_type() == TokensTypes::FLOAT
        && tokens_to_transform[1].get_type() == TokensTypes::DIV
        && tokens_to_transform[2].get_type() == TokensTypes::FLOAT
    {
        return Ok(Token::new(
            format!(
                "{}",
                tokens_to_transform[0].get_value().parse::<f64>().unwrap()
                    / tokens_to_transform[2]
                        .get_value()
                        .parse::<f64>()
                        .ok()
                        .unwrap()
            ),
            TokensTypes::FLOAT,
        ));
    }
    Err(format!(
        "a error in tokens: \n1:{:?}\n2:{:?}\n3:{:?}",
        tokens_to_transform[0], tokens_to_transform[1], tokens_to_transform[2]
    ))
}
pub fn is_math_operation(tokens: Vec<Token>) -> bool {
    if tokens.len() < 3 {
        return false;
    }
    if tokens[0].get_type() != TokensTypes::INT
        && tokens[2].get_type() != TokensTypes::FLOAT
        && tokens[2].get_type() != TokensTypes::INT
        && tokens[2].get_type() != TokensTypes::FLOAT
    {
        return false;
    }
    if tokens[1].get_type() != TokensTypes::PLUS
        && tokens[1].get_type() != TokensTypes::MINUS
        && tokens[1].get_type() != TokensTypes::MULT
        && tokens[1].get_type() != TokensTypes::DIV
    {
        return false;
    }
    true
}
pub fn tranform_logic_comparation_to_result(tokens: Vec<Token>) -> Result<Token, String> {
    /*d */
    let mut true_or_false_number: u8 = 0;

    // 1 = true 0 = false
    if tokens.len() != 3 && tokens.len() != 4 && is_logic_comparation(tokens.clone()) == false {
        return Err(String::from(
            "is logic comparation(true,false) is invalid :/",
        ));
    }

    if tokens.len() == 3 {
        // string comparation ==
        if tokens[0].get_type() == TokensTypes::STRING
            && tokens[1].get_type() == TokensTypes::IF_IS_EQUALS
            && tokens[2].get_type() == TokensTypes::STRING
        {
            if tokens[0].get_value() == tokens[2].get_value() {
                true_or_false_number = 1;
            } else {
                true_or_false_number = 0;
            }
        } else if tokens[0].get_type() == TokensTypes::INT
            && tokens[1].get_type() == TokensTypes::IF_IS_EQUALS
            && tokens[2].get_type() == TokensTypes::INT
        {
            if tokens[0].get_value().parse::<isize>().ok().unwrap()
                == tokens[2].get_value().parse::<isize>().ok().unwrap()
            {
                true_or_false_number = 1;
            } else {
                true_or_false_number = 0;
            }
        } else if tokens[0].get_type() == TokensTypes::FLOAT
            && tokens[1].get_type() == TokensTypes::IF_IS_EQUALS
            && tokens[2].get_type() == TokensTypes::FLOAT
        {
            if tokens[0].get_value().parse::<f64>().ok().unwrap()
                == tokens[2].get_value().parse::<f64>().ok().unwrap()
            {
                true_or_false_number = 1;
            } else {
                true_or_false_number = 0;
            }
        }
        // mayor >
        else if tokens[0].get_type() == TokensTypes::INT
            && tokens[1].get_type() == TokensTypes::IF_IS_GREATER
            && tokens[2].get_type() == TokensTypes::INT
        {
            if tokens[0].get_value().parse::<isize>().ok().unwrap()
                > tokens[2].get_value().parse::<isize>().ok().unwrap()
            {
                true_or_false_number = 1;
            } else {
                true_or_false_number = 0;
            }
        } else if tokens[0].get_type() == TokensTypes::FLOAT
            && tokens[1].get_type() == TokensTypes::IF_IS_GREATER
            && tokens[2].get_type() == TokensTypes::FLOAT
        {
            if tokens[0].get_value().parse::<f64>().ok().unwrap()
                > tokens[2].get_value().parse::<f64>().ok().unwrap()
            {
                true_or_false_number = 1;
            } else {
                true_or_false_number = 0;
            }
        }
        // MENOR <
        else if tokens[0].get_type() == TokensTypes::INT
            && tokens[1].get_type() == TokensTypes::IF_IS_LESS
            && tokens[2].get_type() == TokensTypes::INT
        {
            if tokens[0].get_value().parse::<isize>().ok().unwrap()
                < tokens[2].get_value().parse::<isize>().ok().unwrap()
            {
                true_or_false_number = 1;
            } else {
                true_or_false_number = 0;
            }
        } else if tokens[0].get_type() == TokensTypes::FLOAT
            && tokens[1].get_type() == TokensTypes::IF_IS_LESS
            && tokens[2].get_type() == TokensTypes::FLOAT
        {
            if tokens[0].get_value().parse::<f64>().ok().unwrap()
                < tokens[2].get_value().parse::<f64>().ok().unwrap()
            {
                true_or_false_number = 1;
            } else {
                true_or_false_number = 0;
            }
        }
        // mayor o igual
        else if tokens[0].get_type() == TokensTypes::INT
            && tokens[1].get_type() == TokensTypes::IF_IS_GREATER_EQUALS
            && tokens[2].get_type() == TokensTypes::INT
        {
            if tokens[0].get_value().parse::<isize>().ok().unwrap()
                >= tokens[2].get_value().parse::<isize>().ok().unwrap()
            {
                true_or_false_number = 1;
            } else {
                true_or_false_number = 0;
            }
        } else if tokens[0].get_type() == TokensTypes::FLOAT
            && tokens[1].get_type() == TokensTypes::IF_IS_GREATER_EQUALS
            && tokens[2].get_type() == TokensTypes::FLOAT
        {
            if tokens[0].get_value().parse::<f64>().ok().unwrap()
                >= tokens[2].get_value().parse::<f64>().ok().unwrap()
            {
                true_or_false_number = 1;
            } else {
                true_or_false_number = 0;
            }
        }
        // menor o igual
        else if tokens[0].get_type() == TokensTypes::INT
            && tokens[1].get_type() == TokensTypes::IF_IS_LESS_EQUALS
            && tokens[2].get_type() == TokensTypes::INT
        {
            if tokens[0].get_value().parse::<isize>().ok().unwrap()
                <= tokens[2].get_value().parse::<isize>().ok().unwrap()
            {
                true_or_false_number = 1;
            } else {
                true_or_false_number = 0;
            }
        } else if tokens[0].get_type() == TokensTypes::FLOAT
            && tokens[1].get_type() == TokensTypes::IF_IS_LESS_EQUALS
            && tokens[2].get_type() == TokensTypes::FLOAT
        {
            if tokens[0].get_value().parse::<f64>().ok().unwrap()
                <= tokens[2].get_value().parse::<f64>().ok().unwrap()
            {
                true_or_false_number = 1;
            } else {
                true_or_false_number = 0;
            }
        } else {
            return Err(String::from(format!(
                "is logic comparation(true,false) is invalid :/{:?}",
                tokens
            )));
        }
    }
    //
    else {
        // string comparation ==
        if tokens[0].get_type() == TokensTypes::NOT
            && tokens[1].get_type() == TokensTypes::STRING
            && tokens[2].get_type() == TokensTypes::IF_IS_EQUALS
            && tokens[3].get_type() == TokensTypes::STRING
        {
            if tokens[1].get_type() == TokensTypes::NOT
                && tokens[3].get_value() == tokens[3].get_value()
            {
                true_or_false_number = 0;
            } else {
                true_or_false_number = 1;
            }
        } else if tokens[0].get_type() == TokensTypes::NOT
            && tokens[1].get_type() == TokensTypes::INT
            && tokens[2].get_type() == TokensTypes::IF_IS_EQUALS
            && tokens[3].get_type() == TokensTypes::INT
        {
            if tokens[1].get_value().parse::<isize>().ok().unwrap()
                == tokens[3].get_value().parse::<isize>().ok().unwrap()
            {
                true_or_false_number = 0;
            } else {
                true_or_false_number = 1;
            }
        } else if tokens[0].get_type() == TokensTypes::NOT
            && tokens[1].get_type() == TokensTypes::FLOAT
            && tokens[2].get_type() == TokensTypes::IF_IS_EQUALS
            && tokens[3].get_type() == TokensTypes::FLOAT
        {
            if tokens[1].get_value().parse::<f64>().ok().unwrap()
                == tokens[3].get_value().parse::<f64>().ok().unwrap()
            {
                true_or_false_number = 0;
            } else {
                true_or_false_number = 1;
            }
        }
        // mayor >
        else if tokens[0].get_type() == TokensTypes::NOT
            && tokens[1].get_type() == TokensTypes::INT
            && tokens[2].get_type() == TokensTypes::IF_IS_GREATER
            && tokens[3].get_type() == TokensTypes::INT
        {
            if tokens[1].get_value().parse::<isize>().ok().unwrap()
                > tokens[3].get_value().parse::<isize>().ok().unwrap()
            {
                true_or_false_number = 0;
            } else {
                true_or_false_number = 1;
            }
        } else if tokens[0].get_type() == TokensTypes::NOT
            && tokens[1].get_type() == TokensTypes::FLOAT
            && tokens[2].get_type() == TokensTypes::IF_IS_GREATER
            && tokens[3].get_type() == TokensTypes::FLOAT
        {
            if tokens[1].get_value().parse::<f64>().ok().unwrap()
                > tokens[3].get_value().parse::<f64>().ok().unwrap()
            {
                true_or_false_number = 0;
            } else {
                true_or_false_number = 1;
            }
        }
        // MENOR <
        else if tokens[0].get_type() == TokensTypes::NOT
            && tokens[1].get_type() == TokensTypes::INT
            && tokens[2].get_type() == TokensTypes::IF_IS_LESS
            && tokens[3].get_type() == TokensTypes::INT
        {
            if tokens[1].get_value().parse::<isize>().ok().unwrap()
                < tokens[3].get_value().parse::<isize>().ok().unwrap()
            {
                true_or_false_number = 0;
            } else {
                true_or_false_number = 1;
            }
        } else if tokens[0].get_type() == TokensTypes::NOT
            && tokens[1].get_type() == TokensTypes::FLOAT
            && tokens[2].get_type() == TokensTypes::IF_IS_LESS
            && tokens[3].get_type() == TokensTypes::FLOAT
        {
            if tokens[1].get_value().parse::<f64>().ok().unwrap()
                < tokens[3].get_value().parse::<f64>().ok().unwrap()
            {
                true_or_false_number = 0;
            } else {
                true_or_false_number = 1;
            }
        }
        // mayor o igual
        else if tokens[0].get_type() == TokensTypes::NOT
            && tokens[1].get_type() == TokensTypes::INT
            && tokens[2].get_type() == TokensTypes::IF_IS_GREATER_EQUALS
            && tokens[3].get_type() == TokensTypes::INT
        {
            if tokens[1].get_value().parse::<isize>().ok().unwrap()
                >= tokens[3].get_value().parse::<isize>().ok().unwrap()
            {
                true_or_false_number = 0;
            } else {
                true_or_false_number = 1;
            }
        } else if tokens[0].get_type() == TokensTypes::NOT
            && tokens[1].get_type() == TokensTypes::FLOAT
            && tokens[2].get_type() == TokensTypes::IF_IS_GREATER_EQUALS
            && tokens[3].get_type() == TokensTypes::FLOAT
        {
            if tokens[1].get_value().parse::<f64>().ok().unwrap()
                >= tokens[3].get_value().parse::<f64>().ok().unwrap()
            {
                true_or_false_number = 0;
            } else {
                true_or_false_number = 1;
            }
        }
        // menor o igual
        else if tokens[0].get_type() == TokensTypes::NOT
            && tokens[1].get_type() == TokensTypes::INT
            && tokens[2].get_type() == TokensTypes::IF_IS_LESS_EQUALS
            && tokens[3].get_type() == TokensTypes::INT
        {
            if tokens[1].get_value().parse::<isize>().ok().unwrap()
                <= tokens[3].get_value().parse::<isize>().ok().unwrap()
            {
                true_or_false_number = 0;
            } else {
                true_or_false_number = 1;
            }
        } else if tokens[0].get_type() == TokensTypes::NOT
            && tokens[1].get_type() == TokensTypes::FLOAT
            && tokens[2].get_type() == TokensTypes::IF_IS_LESS_EQUALS
            && tokens[3].get_type() == TokensTypes::FLOAT
        {
            if tokens[1].get_value().parse::<f64>().ok().unwrap()
                <= tokens[3].get_value().parse::<f64>().ok().unwrap()
            {
                true_or_false_number = 0;
            } else {
                true_or_false_number = 1;
            }
        } else {
            return Err(String::from(
                "is logic comparation(true,false) is invalid :/",
            ));
        }
    }

    Ok(Token::new(
        format!("{}", true_or_false_number),
        TokensTypes::INT,
    ))
}
pub fn is_logic_comparation(tokens: Vec<Token>) -> bool {
    // igual equals
    // if 2 == 2
    //if not 2 == 2
    if tokens.len() != 3 && tokens.len() != 4 {
        return false;
    }

    // new

    if tokens.len() == 3 {
        if tokens[0].get_type() != TokensTypes::STRING
            && tokens[0].get_type() != TokensTypes::INT
            && tokens[0].get_type() != TokensTypes::FLOAT
        {
            return false;
        }
        if tokens[1].get_type() != TokensTypes::IF_IS_EQUALS
            && tokens[1].get_type() != TokensTypes::IF_IS_GREATER
            && tokens[1].get_type() != TokensTypes::IF_IS_GREATER_EQUALS
            && tokens[1].get_type() != TokensTypes::IF_IS_LESS
            && tokens[1].get_type() != TokensTypes::IF_IS_LESS_EQUALS
        {
            return false;
        }
        if tokens[2].get_type() != TokensTypes::STRING
            && tokens[2].get_type() != TokensTypes::INT
            && tokens[2].get_type() != TokensTypes::FLOAT
        {
            return false;
        }
    }

    if tokens.len() == 4 {
        if tokens[0].get_type() != TokensTypes::NOT {
            return false;
        }
        if tokens[1].get_type() != TokensTypes::STRING
            && tokens[1].get_type() != TokensTypes::INT
            && tokens[1].get_type() != TokensTypes::FLOAT
        {
            return false;
        }
        if tokens[2].get_type() != TokensTypes::IF_IS_EQUALS
            && tokens[2].get_type() != TokensTypes::IF_IS_GREATER
            && tokens[2].get_type() != TokensTypes::IF_IS_GREATER_EQUALS
            && tokens[2].get_type() != TokensTypes::IF_IS_LESS
            && tokens[2].get_type() != TokensTypes::IF_IS_LESS_EQUALS
        {
            return false;
        }
        if tokens[3].get_type() != TokensTypes::STRING
            && tokens[3].get_type() != TokensTypes::INT
            && tokens[3].get_type() != TokensTypes::FLOAT
        {
            return false;
        }
    }

    true
}
pub fn interpretate(
    mut tokens: Vec<Token>,
    mut vars: HashMap<String, Token>,
) -> HashMap<String, Token> {
    tokens = Token::ignore_space_tokens(tokens);
    //let mut rust_code: String = "".to_string();
    let mut i: usize = 0;
    let mut err_txt: String = "".to_string();
    let mut is_only_line: bool = true;
    let mut j: usize = 0;
    let mut number_lines: usize = 0;
    while j < tokens.len() {
        if tokens[j].get_type() == TokensTypes::END_LINE {
            number_lines += 1;
        }
        j += 1;
    }
    if number_lines > 1 {
        is_only_line = false;
    }
    // println!("                        {}", is_only_line);
    while i < tokens.len() {
        let mut current_token: Token = tokens[i].clone();
        if current_token.get_type() != TokensTypes::END_LINE {
            //println!("{:#?}", current_token);

            // LOGICS
            if is_only_line {
                let mut k: usize = i;
                while k < tokens.len() {
                    let mut putin: bool = false;
                    if k + 2 < tokens.len() {
                        let mut vec_t99: Vec<Token> = Vec::new();
                        vec_t99.push(tokens[k].clone());
                        vec_t99.push(tokens[k + 1].clone());
                        vec_t99.push(tokens[k + 2].clone());
                        if is_math_operation(vec_t99.clone()) {
                            let mut h40: Result<Token, String> = tranform_math_to_result(vec_t99);
                            if (h40.is_err()) {
                                err_txt = h40.err().unwrap();
                            } else {
                                tokens[k] = h40.ok().unwrap();
                                tokens[k + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                                tokens[k + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
                                putin = true;
                                tokens = Token::ignore_space_tokens(tokens);
                                current_token = tokens[i].clone();
                            }
                        }
                    }
                    if !putin {
                        k += 1;
                    }
                }
                let mut k51: usize = i;
                //3 == 3
                while k51 < tokens.len() {
                    let mut putin: bool = false;
                    if k51 + 2 < tokens.len() {
                        let mut vec_t99: Vec<Token> = Vec::new();
                        vec_t99.push(tokens[k51].clone());
                        vec_t99.push(tokens[k51 + 1].clone());
                        vec_t99.push(tokens[k51 + 2].clone());
                        //println!("{}", is_logic_comparation(vec_t99.clone()));
                        if is_logic_comparation(vec_t99.clone()) {
                            let mut result3 = tranform_logic_comparation_to_result(vec_t99.clone());
                            if result3.is_ok() {
                                tokens[k51] = result3.ok().unwrap();
                                tokens[k51 + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                                tokens[k51 + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
                                tokens = Token::ignore_space_tokens(tokens);
                                current_token = tokens[i].clone();
                                putin = true;
                            } else {
                                err_txt = result3.err().unwrap();
                            }
                        }
                    }
                    if k51 + 3 < tokens.len() {
                        let mut vec_t99: Vec<Token> = Vec::new();
                        vec_t99.push(tokens[k51].clone());
                        vec_t99.push(tokens[k51 + 1].clone());
                        vec_t99.push(tokens[k51 + 2].clone());
                        vec_t99.push(tokens[k51 + 3].clone());
                        //println!("{}", is_logic_comparation(vec_t99.clone()));
                        if is_logic_comparation(vec_t99.clone()) {
                            let mut result3 = tranform_logic_comparation_to_result(vec_t99.clone());
                            if result3.is_ok() {
                                tokens[k51] = result3.ok().unwrap();
                                tokens[k51 + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                                tokens[k51 + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
                                tokens[k51 + 3] = Token::new(" ".to_string(), TokensTypes::SPACE);
                                tokens = Token::ignore_space_tokens(tokens);
                                current_token = tokens[i].clone();
                                putin = true;
                            } else {
                                err_txt = result3.err().unwrap();
                            }
                        }
                    }
                    if !putin {
                        k51 += 1;
                    }
                }
            } else {
                let mut k20: usize = i;
                while k20.clone() < tokens.len() && tokens[k20].get_type() != TokensTypes::END_LINE
                {
                    //println!("index k:{}", k);
                    if k20 + 1 > tokens.len() {
                        break;
                    } else {
                        if tokens[k20].get_type() == TokensTypes::TEXT
                            && tokens[k20 + 1].get_type() != TokensTypes::EQUAL
                        {
                            if vars.get(tokens[k20].get_value().as_str()).is_some() {
                                let name_of_var: String = tokens[k20].get_value();
                                tokens[k20] = vars.get(name_of_var.as_str()).unwrap().clone();
                            }
                        }
                    }

                    k20 += 1;
                }
            }
            tokens = Token::ignore_space_tokens(tokens);
            current_token = tokens[i].clone();
            let mut k: usize = i;
            while k.clone() < tokens.len() && tokens[k].get_type() != TokensTypes::END_LINE {
                //println!("index k:{}", k);
                let mut putin: bool = false;
                if k + 1 >= tokens.len() {
                    break;
                }
                if k + 2 < tokens.len() {
                    let mut vec_t99: Vec<Token> = Vec::new();
                    vec_t99.push(tokens[k].clone());
                    vec_t99.push(tokens[k + 1].clone());
                    vec_t99.push(tokens[k + 2].clone());
                    if is_math_operation(vec_t99.clone()) {
                        let mut h40: Result<Token, String> = tranform_math_to_result(vec_t99);
                        if (h40.is_err()) {
                            err_txt = h40.err().unwrap();
                        } else {
                            tokens[k] = h40.ok().unwrap();
                            tokens[k + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                            tokens[k + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
                            putin = true;
                            tokens = Token::ignore_space_tokens(tokens);
                            current_token = tokens[i].clone();
                        }
                    }
                }

                if !putin {
                    k += 1;
                }
            }

            tokens = Token::ignore_space_tokens(tokens);
            current_token = tokens[i].clone();
            let mut k51: usize = i;
            //3 == 3
            while k51.clone() < tokens.len() && tokens[k51].get_type() != TokensTypes::END_LINE {
                let mut putin: bool = false;
                if k51 + 2 < tokens.len() {
                    let mut vec_t99: Vec<Token> = Vec::new();
                    vec_t99.push(tokens[k51].clone());
                    vec_t99.push(tokens[k51 + 1].clone());
                    vec_t99.push(tokens[k51 + 2].clone());
                    if is_logic_comparation(vec_t99.clone()) {
                        let mut result3 = tranform_logic_comparation_to_result(vec_t99.clone());
                        if result3.is_ok() {
                            tokens[k51] = result3.ok().unwrap();
                            tokens[k51 + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                            tokens[k51 + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
                            tokens = Token::ignore_space_tokens(tokens);
                            current_token = tokens[i].clone();
                            putin = true;
                        } else {
                            err_txt = result3.err().unwrap();
                        }
                    }
                }
                if k51 + 3 < tokens.len() {
                    let mut vec_t99: Vec<Token> = Vec::new();
                    vec_t99.push(tokens[k51].clone());
                    vec_t99.push(tokens[k51 + 1].clone());
                    vec_t99.push(tokens[k51 + 2].clone());
                    vec_t99.push(tokens[k51 + 3].clone());
                    //println!("{}", is_logic_comparation(vec_t99.clone()));
                    if is_logic_comparation(vec_t99.clone()) {
                        let mut result3 = tranform_logic_comparation_to_result(vec_t99.clone());
                        if result3.is_ok() {
                            tokens[k51] = result3.ok().unwrap();
                            tokens[k51 + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                            tokens[k51 + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
                            tokens[k51 + 3] = Token::new(" ".to_string(), TokensTypes::SPACE);
                            tokens = Token::ignore_space_tokens(tokens);
                            current_token = tokens[i].clone();
                            putin = true;
                        } else {
                            err_txt = result3.err().unwrap();
                        }
                    }
                }
                if !putin {
                    k51 += 1;
                }
            }
            tokens = Token::ignore_space_tokens(tokens);
            current_token = tokens[i].clone();
            // println!(
            //     "TOKENS IMPORTANTES AAAAAAAAAAAAAAAREVISAR :B \n{:#?} \nYA TETETTETETTETE",
            //     tokens.clone()
            // );
            if i + 2 < tokens.len() {
                let mut vec_t99: Vec<Token> = Vec::new();
                vec_t99.push(tokens[i].clone());
                vec_t99.push(tokens[i + 1].clone());
                vec_t99.push(tokens[i + 2].clone());

                if is_math_operation(vec_t99.clone()) {
                    let mut h40: Result<Token, String> = tranform_math_to_result(vec_t99);
                    if (h40.is_err()) {
                        err_txt = h40.err().unwrap();
                    } else {
                        tokens[i] = h40.ok().unwrap();
                        tokens[i + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                        tokens[i + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
                    }
                }
            }
            tokens = Token::ignore_space_tokens(tokens);
            current_token = tokens[i].clone();

            // BUILD FUNCTIONS AND MORE

            if current_token.get_type() == TokensTypes::TEXT
                && tokens[i + 1].get_type() == TokensTypes::EQUAL
            {
                if vars.contains_key(current_token.get_value().as_str()) {
                    if tokens[i + 2].get_type()
                        != vars
                            .get(current_token.get_value().as_str())
                            .unwrap()
                            .get_type()
                    {
                        err_txt = "TYPE ERROR".to_string();
                    } else {
                        vars.insert(current_token.get_value(), tokens[i + 2].clone());
                    }
                } else {
                    err_txt= "The variable does not exist. You cannot assign the value to something that does not exist".to_string();
                }
                i += 3;
            } else if current_token.get_type() == TokensTypes::STRING
                || current_token.get_type() == TokensTypes::INT
                || current_token.get_type() == TokensTypes::FLOAT
            {
            } else if current_token.get_type() == TokensTypes::TEXT
                && tokens[i + 1].get_type() == TokensTypes::L_PARENT
            {
                // BUILD FUNCTIONS
                if current_token.get_value() == "print_text" {
                    if tokens[i + 2].get_type() != TokensTypes::STRING {
                        err_txt = "invalid type in print_text function :(".to_string();
                    } else {
                        if tokens[i + 3].get_type() != TokensTypes::R_PARENT {
                            err_txt = "sintax error in print_text function".to_string();
                        } else {
                            print!("{}", tokens[i + 2].get_value());
                            i += 3;
                        }
                    }
                }
                if current_token.get_value() == "print_string" {
                    if tokens[i + 2].get_type() != TokensTypes::STRING {
                        err_txt = "invalid type in print_string function :(".to_string();
                    } else {
                        if tokens[i + 3].get_type() != TokensTypes::R_PARENT {
                            err_txt = "sintax error in print_string function".to_string();
                        } else {
                            print!("{}", tokens[i + 2].get_value());
                            i += 3;
                        }
                    }
                }
                if current_token.get_value() == "println_string" {
                    if tokens[i + 2].get_type() != TokensTypes::STRING {
                        err_txt = "invalid type in println_string function :(".to_string();
                    } else {
                        if tokens[i + 3].get_type() != TokensTypes::R_PARENT {
                            err_txt = "sintax error in println_string function".to_string();
                        } else {
                            println!("{}", tokens[i + 2].get_value());
                            i += 3;
                        }
                    }
                }
                if current_token.get_value() == "println" {
                    if tokens[i + 2].get_type() != TokensTypes::STRING
                        && tokens[i + 2].get_type() != TokensTypes::INT
                        && tokens[i + 2].get_type() != TokensTypes::FLOAT
                    {
                        err_txt = "invalid type in println function :(".to_string();
                    } else {
                        if tokens[i + 3].get_type() != TokensTypes::R_PARENT {
                            err_txt = "sintax error in println function".to_string();
                        } else {
                            println!("{}", tokens[i + 2].get_value());
                            i += 3;
                        }
                    }
                }
                if current_token.get_value() == "print" {
                    if tokens[i + 2].get_type() != TokensTypes::STRING
                        && tokens[i + 2].get_type() != TokensTypes::INT
                        && tokens[i + 2].get_type() != TokensTypes::FLOAT
                    {
                        err_txt = "invalid type in print function :(".to_string();
                    } else {
                        if tokens[i + 3].get_type() != TokensTypes::R_PARENT {
                            err_txt = "sintax error in print function".to_string();
                        } else {
                            print!("{}", tokens[i + 2].get_value());
                            i += 3;
                        }
                    }
                }
                if current_token.get_value() == "print_int" {
                    if tokens[i + 2].get_type() != TokensTypes::INT {
                        err_txt = "invalid type in print_int function :(".to_string();
                    } else {
                        if tokens[i + 3].get_type() != TokensTypes::R_PARENT {
                            err_txt = "sintax error in print_int function".to_string();
                        } else {
                            print!("{}", tokens[i + 2].get_value());
                            i += 3;
                        }
                    }
                }
                if current_token.get_value() == "print_float" {
                    if tokens[i + 2].get_type() != TokensTypes::FLOAT {
                        err_txt = "invalid type in print_float function :(".to_string();
                    } else {
                        if tokens[i + 3].get_type() != TokensTypes::R_PARENT {
                            err_txt = "sintax error in print_float function".to_string();
                        } else {
                            print!("{}", tokens[i + 2].get_value());
                            i += 3;
                        }
                    }
                }
                if current_token.get_value() == "println_text" {
                    if tokens[i + 2].get_type() != TokensTypes::STRING {
                        err_txt = "invalid type in println_text function :(".to_string();
                    } else {
                        if tokens[i + 3].get_type() != TokensTypes::R_PARENT {
                            err_txt = "sintax error in println_text function".to_string();
                        } else {
                            println!("{}", tokens[i + 2].get_value());
                            i += 3;
                        }
                    }
                } else if current_token.get_value() == "println_int" {
                    if tokens[i + 2].get_type() != TokensTypes::INT {
                        err_txt = "invalid type in println_int function :(".to_string();
                    } else {
                        if tokens[i + 3].get_type() != TokensTypes::R_PARENT {
                            err_txt = "sintax error in println_int function".to_string();
                        } else {
                            println!("{}", tokens[i + 2].get_value());
                            i += 3;
                        }
                    }
                } else if current_token.get_value() == "println_float" {
                    if tokens[i + 2].get_type() != TokensTypes::FLOAT {
                        err_txt = "invalid type in println_float function :(".to_string();
                    } else {
                        if tokens[i + 3].get_type() != TokensTypes::R_PARENT {
                            err_txt = "sintax error in println_float function".to_string();
                        } else {
                            println!("{}", tokens[i + 2].get_value());
                            i += 3;
                        }
                    }
                }
            } else if current_token.get_type() == TokensTypes::LET_VAR {
                if vars.contains_key(tokens[i + 1].get_value().as_str()) {
                    err_txt = "the variable already exists".to_string();
                } else {
                    vars.insert(tokens[i + 1].get_value(), tokens[i + 5].clone());
                }
                i += 5;
            } else {
                err_txt = "error in function".to_string();
            }
        }

        if err_txt != "" {
            break;
        }

        i += 1;
    }
    if err_txt != "" {
        println!(" \nRUNTIME ERROR :(\n {}\n", err_txt);
    }
    //println!("{:#?}", tokens);

    vars
}
