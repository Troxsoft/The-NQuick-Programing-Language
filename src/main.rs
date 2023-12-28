use std::{collections::HashMap, env, fs, time::Instant};

use Language::{
    format_and_clean_code,
    GenerateCode::{get_if_tokens, interpretate},
    Lexer::Lexer,
    LexerError::LexerError,
    Token::Token,
};

fn main() {
    let start_time = Instant::now();
    let args: Vec<_> = env::args().collect();
    if args.len() > 3 {
        println!("error in arguments :(");
    }
    if args.len() < 2 {
        println!("The first argument not exist");
    } else {
        if args.len() == 3 {
            if args[1] == "time" {
                let code: String = format_and_clean_code(
                    fs::read_to_string(args[2].clone().to_string())
                        .unwrap()
                        .to_string(),
                );

                let mut lexer: Lexer = Lexer::new(code.clone());
                let mut tokens1: Vec<Token> = lexer.run_laxer();
                let mut tokens2: Vec<Token> = lexer.run_laxer2(tokens1);
                // println!("{:#?}", tokens2.clone());
                let mut errors: Vec<LexerError> = lexer.get_sintaxis_error(tokens2.clone());
                if errors.len() >= 1 {
                    println!("errors .not working{:#?}", errors);
                } else {
                    let mut vars: HashMap<String, Token> = HashMap::new();
                    vars.insert(
                        "true".to_string(),
                        Token::new("0".to_string(), Language::Token::TokensTypes::INT_TYPE),
                    );
                    vars.insert(
                        "false".to_string(),
                        Token::new("1".to_string(), Language::Token::TokensTypes::INT_TYPE),
                    );
                    // println!("{:?}", interpretate(tokens2.clone(), vars));
                    interpretate(tokens2.clone(), vars);
                    let elapsed = start_time.elapsed();

                    println!(
                        "interpreted in \n millisegunds: {}\n segunds: {}\n nano-segunds: {}",
                        elapsed.as_millis(),
                        elapsed.as_secs(),
                        elapsed.as_nanos()
                    );
                }
            } else {
                println!("error");
            }
        } else {
            let code: String = format_and_clean_code(
                fs::read_to_string(args[1].clone().to_string())
                    .unwrap()
                    .to_string(),
            );

            let mut lexer: Lexer = Lexer::new(code.clone());
            let mut tokens1: Vec<Token> = lexer.run_laxer();
            let mut tokens2: Vec<Token> = lexer.run_laxer2(tokens1);
            // println!("{:#?}", tokens2.clone());
            let mut errors: Vec<LexerError> = lexer.get_sintaxis_error(tokens2.clone());
            if errors.len() >= 1 {
                println!("errors .not working{:#?}", errors);
            } else {
                let mut vars: HashMap<String, Token> = HashMap::new();
                vars.insert(
                    "true".to_string(),
                    Token::new("1".to_string(), Language::Token::TokensTypes::INT),
                );
                vars.insert(
                    "false".to_string(),
                    Token::new("0".to_string(), Language::Token::TokensTypes::INT),
                );
                // println!("{:?}", interpretate(tokens2.clone(), vars));
                interpretate(tokens2.clone(), vars);
                //println!("{:?}",get_if_tokens())
            }
        }

        //println!("{}", code.clone());
    }
}
