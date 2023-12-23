use std::{collections::HashMap, fs};

use Language::{
    format_and_clean_code, GenerateCode::interpretate, Lexer::Lexer, LexerError::LexerError,
    Token::Token,
};

fn main() {
    let code: String = format_and_clean_code(fs::read_to_string("test.nq").ok().unwrap());

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
        let mut varfff = interpretate(tokens2.clone(), vars);
        // println!("{:#?}", varfff);
    }

    //println!("{}", code.clone());
}
