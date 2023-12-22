use std::{collections::HashMap, fs, process::Command};

use crate::Token::{Token, TokensTypes};

pub fn interpretate(
    mut tokens: Vec<Token>,
    mut vars: HashMap<String, Token>,
) -> HashMap<String, Token> {
    tokens = Token::ignore_space_tokens(tokens);
    //let mut rust_code: String = "".to_string();
    let mut i: usize = 0;
    let mut err_txt: String = "".to_string();
    while i < tokens.len() {
        let mut current_token: Token = tokens[i].clone();
        if current_token.get_type() != TokensTypes::END_LINE {
            //println!("{:#?}", current_token);

            // LOGICS
            if current_token.get_type() == TokensTypes::INT
                && tokens[i + 1].get_type() == TokensTypes::PLUS
                && tokens[i + 2].get_type() == TokensTypes::INT
            {
                current_token = Token::new(
                    format!(
                        "{}",
                        tokens[i].get_value().parse::<isize>().ok().unwrap()
                            + tokens[i + 2].get_value().parse::<isize>().ok().unwrap()
                    ),
                    TokensTypes::INT,
                );
                tokens[i] = current_token.clone();
                tokens[i + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                tokens[i + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
            }
            if current_token.get_type() == TokensTypes::INT
                && tokens[i + 1].get_type() == TokensTypes::MINUS
                && tokens[i + 2].get_type() == TokensTypes::INT
            {
                current_token = Token::new(
                    format!(
                        "{}",
                        tokens[i].get_value().parse::<isize>().ok().unwrap()
                            - tokens[i + 2].get_value().parse::<isize>().ok().unwrap()
                    ),
                    TokensTypes::INT,
                );
                tokens[i] = current_token.clone();
                tokens[i + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                tokens[i + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
            }
            if current_token.get_type() == TokensTypes::INT
                && tokens[i + 1].get_type() == TokensTypes::MULT
                && tokens[i + 2].get_type() == TokensTypes::INT
            {
                current_token = Token::new(
                    format!(
                        "{}",
                        tokens[i].get_value().parse::<isize>().ok().unwrap()
                            * tokens[i + 2].get_value().parse::<isize>().ok().unwrap()
                    ),
                    TokensTypes::INT,
                );
                tokens[i] = current_token.clone();
                tokens[i + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                tokens[i + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
            }
            if current_token.get_type() == TokensTypes::INT
                && tokens[i + 1].get_type() == TokensTypes::DIV
                && tokens[i + 2].get_type() == TokensTypes::INT
            {
                current_token = Token::new(
                    format!(
                        "{}",
                        tokens[i].get_value().parse::<isize>().ok().unwrap()
                            / tokens[i + 2].get_value().parse::<isize>().ok().unwrap()
                    ),
                    TokensTypes::INT,
                );
                tokens[i] = current_token.clone();
                tokens[i + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                tokens[i + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
            }

            if current_token.get_type() == TokensTypes::FLOAT
                && tokens[i + 1].get_type() == TokensTypes::PLUS
                && tokens[i + 2].get_type() == TokensTypes::FLOAT
            {
                current_token = Token::new(
                    format!(
                        "{}",
                        tokens[i].get_value().parse::<f64>().unwrap()
                            + tokens[i + 2].get_value().parse::<f64>().ok().unwrap()
                    ),
                    TokensTypes::FLOAT,
                );
                tokens[i] = current_token.clone();
                tokens[i + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                tokens[i + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
            }
            if current_token.get_type() == TokensTypes::FLOAT
                && tokens[i + 1].get_type() == TokensTypes::MINUS
                && tokens[i + 2].get_type() == TokensTypes::FLOAT
            {
                current_token = Token::new(
                    format!(
                        "{}",
                        tokens[i].get_value().parse::<f64>().ok().unwrap()
                            - tokens[i + 2].get_value().parse::<f64>().ok().unwrap()
                    ),
                    TokensTypes::FLOAT,
                );
                tokens[i] = current_token.clone();
                tokens[i + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                tokens[i + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
            }
            if current_token.get_type() == TokensTypes::FLOAT
                && tokens[i + 1].get_type() == TokensTypes::MULT
                && tokens[i + 2].get_type() == TokensTypes::FLOAT
            {
                current_token = Token::new(
                    format!(
                        "{}",
                        tokens[i].get_value().parse::<f64>().ok().unwrap()
                            * tokens[i + 2].get_value().parse::<f64>().ok().unwrap()
                    ),
                    TokensTypes::FLOAT,
                );
                tokens[i] = current_token.clone();
                tokens[i + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                tokens[i + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
            }
            if current_token.get_type() == TokensTypes::FLOAT
                && tokens[i + 1].get_type() == TokensTypes::DIV
                && tokens[i + 2].get_type() == TokensTypes::FLOAT
            {
                current_token = Token::new(
                    format!(
                        "{}",
                        tokens[i].get_value().parse::<f64>().ok().unwrap()
                            / tokens[i + 2].get_value().parse::<f64>().ok().unwrap()
                    ),
                    TokensTypes::FLOAT,
                );
                tokens[i] = current_token.clone();
                tokens[i + 1] = Token::new(" ".to_string(), TokensTypes::SPACE);
                tokens[i + 2] = Token::new(" ".to_string(), TokensTypes::SPACE);
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
                if current_token.get_value() == "write_text" {
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
                if current_token.get_value() == "writeln_text" {
                    if tokens[i + 2].get_type() != TokensTypes::STRING {
                        err_txt = "invalid type in print_text function :(".to_string();
                    } else {
                        if tokens[i + 3].get_type() != TokensTypes::R_PARENT {
                            err_txt = "sintax error in print_text function".to_string();
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
        println!(" \nERROR\n {}\n", err_txt);
    }
    println!("{:#?}", tokens);
    vars
}
