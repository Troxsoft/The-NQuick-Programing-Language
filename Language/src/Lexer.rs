use core::num;
use std::vec;

use crate::{
    LexerError::LexerError,
    Token::{Token, TokensTypes},
};

#[derive(Debug, Clone)]
pub struct Lexer {
    text: String,
    position: usize,
}
impl Lexer {
    pub fn new(text: String) -> Self {
        return Lexer {
            position: 0,
            text: text,
        };
    }
    fn get_current_char(&self) -> String {
        self.text
            .chars()
            .nth(self.position)
            .unwrap()
            .to_string()
            .clone()
    }
    fn next_position(&mut self) -> Option<()> {
        if self.is_final() == false {
            self.position += 1;
            Some(())
        } else {
            None
        }
    }

    fn is_final(&self) -> bool {
        if self.position.clone() < self.text.len() {
            false
        } else {
            true
        }
    }
    pub fn is_number_current_char(&mut self) -> bool {
        if self.get_current_char() == "0"
            || self.get_current_char() == "1"
            || self.get_current_char() == "2"
            || self.get_current_char() == "3"
            || self.get_current_char() == "4"
            || self.get_current_char() == "5"
            || self.get_current_char() == "6"
            || self.get_current_char() == "7"
            || self.get_current_char() == "8"
            || self.get_current_char() == "9"
        {
            return true;
        }

        false
    }

    pub fn is_number_current_char_txt(&mut self, text: String) -> bool {
        if text == "0"
            || text == "1"
            || text == "2"
            || text == "3"
            || text == "4"
            || text == "5"
            || text == "6"
            || text == "7"
            || text == "8"
            || text == "9"
        {
            return true;
        }

        false
    }
    pub fn is_var_definition(&mut self) -> bool {
        if self.position + 2 < self.text.len()
            && self.get_current_char() == "l"
            && self
                .text
                .chars()
                .nth(self.position + 1)
                .unwrap()
                .to_string()
                == "e"
            && self
                .text
                .chars()
                .nth(self.position + 2)
                .unwrap()
                .to_string()
                == "t"
        {
            return true;
        }
        false
    }

    pub fn create_numbers(&mut self) -> Token {
        let mut number: String = "".to_string();
        let mut mula: bool = false;
        while mula == false && self.is_final() == false && self.is_number_current_char() == true {
            number.push_str(&self.get_current_char());
            if self.text.chars().nth(self.position + 1).is_some()
                && self.is_number_current_char_txt(
                    self.text
                        .chars()
                        .nth(self.position + 1)
                        .unwrap()
                        .to_string(),
                ) == true
            {
                self.next_position();
            } else {
                mula = true;
            }
        }

        return Token::new(number, TokensTypes::INT);
    }
    pub fn run_laxer2(&mut self, preTokens: Vec<Token>) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut i: usize = 0;
        while i < preTokens.len() {
            let current_pre_token = preTokens[i].clone();
            if preTokens.len() > i + 2
                && current_pre_token.get_type().clone() == TokensTypes::INT
                && preTokens[i + 1].get_type().clone() == TokensTypes::POINT
                && preTokens[i + 2].get_type().clone() == TokensTypes::INT
            {
                tokens.push(Token::new(
                    format!(
                        "{}.{}",
                        current_pre_token.clone().get_value(),
                        preTokens[i + 2].clone().get_value()
                    ),
                    TokensTypes::FLOAT,
                ));
                i += 2;
            } else {
                if current_pre_token.get_type() == TokensTypes::TEXT {
                    let mut textd: String = String::new();

                    while i < preTokens.len() && preTokens[i].get_type() == TokensTypes::TEXT {
                        textd.push_str(preTokens[i].get_value().as_str());
                        if i + 1 < preTokens.len()
                            && preTokens[i + 1].get_type() == TokensTypes::TEXT
                        {
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::new(textd, TokensTypes::TEXT));
                } else {
                    tokens.push(current_pre_token);
                }
            }
            i += 1;
        }
        tokens
    }
    pub fn get_sintaxis_error(&mut self, mut tokens: Vec<Token>) -> Vec<LexerError> {
        let mut err: Vec<LexerError> = Vec::new();
        let mut i: usize = 0;
        tokens = Token::ignore_space_tokens(tokens);
        while i < tokens.len() {
            let currentToken: Token = tokens[i].clone();
            if tokens.len() > i + 1
                && currentToken.get_type() == TokensTypes::POINT
                && tokens[i + 1].get_type() == TokensTypes::POINT
            {
                err.push(LexerError::new(
                    "two points ??? ".to_string(),
                    "..".to_string(),
                ));
                i += 1;
            } else if tokens.len() > i + 1
                && currentToken.get_type() == TokensTypes::FLOAT
                && tokens[i + 1].get_type() == TokensTypes::POINT
            {
                err.push(LexerError::new(
                    "the float sintaxis is err.".to_string(),
                    format!("{}{}", currentToken.get_value(), tokens[i + 1].get_value()),
                ));
                i += 1;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::FLOAT
                && tokens[i + 1].get_type() == TokensTypes::PLUS
                && tokens[i + 2].get_type() == TokensTypes::STRING
            {
                err.push(LexerError::new(
                    "cannot be added with text".to_string(),
                    format!("{}+{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::FLOAT
                && tokens[i + 1].get_type() == TokensTypes::MINUS
                && tokens[i + 2].get_type() == TokensTypes::STRING
            {
                err.push(LexerError::new(
                    "cannot be subtracted with text".to_string(),
                    format!("{}-{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::FLOAT
                && tokens[i + 1].get_type() == TokensTypes::MULT
                && tokens[i + 2].get_type() == TokensTypes::STRING
            {
                err.push(LexerError::new(
                    "cannot be multiplied with text".to_string(),
                    format!("{}*{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::FLOAT
                && tokens[i + 1].get_type() == TokensTypes::DIV
                && tokens[i + 2].get_type() == TokensTypes::STRING
            {
                err.push(LexerError::new(
                    "can't divid with text".to_string(),
                    format!("{}/{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::INT
                && tokens[i + 1].get_type() == TokensTypes::PLUS
                && tokens[i + 2].get_type() == TokensTypes::STRING
            {
                err.push(LexerError::new(
                    "cannot be added with text".to_string(),
                    format!("{}+{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::INT
                && tokens[i + 1].get_type() == TokensTypes::MINUS
                && tokens[i + 2].get_type() == TokensTypes::STRING
            {
                err.push(LexerError::new(
                    "cannot be subtracted with text".to_string(),
                    format!("{}-{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::INT
                && tokens[i + 1].get_type() == TokensTypes::MULT
                && tokens[i + 2].get_type() == TokensTypes::STRING
            {
                err.push(LexerError::new(
                    "cannot be multiplied with text".to_string(),
                    format!("{}*{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::INT
                && tokens[i + 1].get_type() == TokensTypes::DIV
                && tokens[i + 2].get_type() == TokensTypes::STRING
            {
                err.push(LexerError::new(
                    "can't divid with text".to_string(),
                    format!("{}/{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::STRING
                && tokens[i + 1].get_type() == TokensTypes::PLUS
                && tokens[i + 2].get_type() == TokensTypes::FLOAT
            {
                err.push(LexerError::new(
                    "cannot be added with text".to_string(),
                    format!("{}+{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::STRING
                && tokens[i + 1].get_type() == TokensTypes::MINUS
                && tokens[i + 2].get_type() == TokensTypes::FLOAT
            {
                err.push(LexerError::new(
                    "cannot be subtracted with text".to_string(),
                    format!("{}-{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::STRING
                && tokens[i + 1].get_type() == TokensTypes::MULT
                && tokens[i + 2].get_type() == TokensTypes::FLOAT
            {
                err.push(LexerError::new(
                    "cannot be multiplied with text".to_string(),
                    format!("{}*{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::STRING
                && tokens[i + 1].get_type() == TokensTypes::DIV
                && tokens[i + 2].get_type() == TokensTypes::FLOAT
            {
                err.push(LexerError::new(
                    "can't divid with text".to_string(),
                    format!("{}/{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::STRING
                && tokens[i + 1].get_type() == TokensTypes::PLUS
                && tokens[i + 2].get_type() == TokensTypes::INT
            {
                err.push(LexerError::new(
                    "cannot be added with text".to_string(),
                    format!("{}+{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::STRING
                && tokens[i + 1].get_type() == TokensTypes::MINUS
                && tokens[i + 2].get_type() == TokensTypes::INT
            {
                err.push(LexerError::new(
                    "cannot be subtracted with text".to_string(),
                    format!("{}-{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::STRING
                && tokens[i + 1].get_type() == TokensTypes::MULT
                && tokens[i + 2].get_type() == TokensTypes::INT
            {
                err.push(LexerError::new(
                    "cannot be multiplied with text".to_string(),
                    format!("{}*{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::STRING
                && tokens[i + 1].get_type() == TokensTypes::DIV
                && tokens[i + 2].get_type() == TokensTypes::INT
            {
                err.push(LexerError::new(
                    "can't divid with text".to_string(),
                    format!("{}/{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::STRING
                && tokens[i + 1].get_type() == TokensTypes::PLUS
                && tokens[i + 2].get_type() == TokensTypes::STRING
            {
                err.push(LexerError::new(
                    "cannot be added with text".to_string(),
                    format!("{}+{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::STRING
                && tokens[i + 1].get_type() == TokensTypes::MINUS
                && tokens[i + 2].get_type() == TokensTypes::STRING
            {
                err.push(LexerError::new(
                    "cannot be subtracted with text".to_string(),
                    format!("{}-{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::STRING
                && tokens[i + 1].get_type() == TokensTypes::MULT
                && tokens[i + 2].get_type() == TokensTypes::STRING
            {
                err.push(LexerError::new(
                    "cannot be multiplied with text".to_string(),
                    format!("{}*{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if tokens.len() > i + 2
                && currentToken.get_type() == TokensTypes::STRING
                && tokens[i + 1].get_type() == TokensTypes::DIV
                && tokens[i + 2].get_type() == TokensTypes::STRING
            {
                err.push(LexerError::new(
                    "can't divid with text".to_string(),
                    format!("{}/{}", currentToken.get_value(), tokens[i + 2].get_value()),
                ));
                i += 2;
            } else if currentToken.get_type() == TokensTypes::IF_CONDITION && i + 3 >= tokens.len()
            {
                err.push(LexerError::new(
                    "error in if declaration".to_string(),
                    format!("{} :(", currentToken.get_value()),
                ));
            } else if currentToken.get_type() == TokensTypes::LET_VAR && i + 5 >= tokens.len() {
                err.push(LexerError::new(
                    "error in variable declaration. [let example:int = 4]".to_string(),
                    format!("{} :(", currentToken.get_value()),
                ));
            } else if currentToken.get_type() == TokensTypes::FUNCTION && i + 5 < tokens.len() {
                err.push(LexerError::new(
                    "error in function declaration.".to_string(),
                    format!("{} :(", currentToken.get_value()),
                ));
            } else if currentToken.get_type() == TokensTypes::LET_VAR
                && tokens[i + 1].get_type() != TokensTypes::TEXT
            {
                err.push(LexerError::new(
                    "error in variable declaration. [let example:int = 4]".to_string(),
                    format!("{} :(", currentToken.get_value()),
                ));
                i += 5;
            } else if currentToken.get_type() == TokensTypes::LET_VAR
                && tokens[i + 2].get_type() != TokensTypes::TWO_POINTS
            {
                err.push(LexerError::new(
                    "error in variable declaration. [let example:int = 4]".to_string(),
                    format!("{} :(", currentToken.get_value()),
                ));
                i += 5;
            } else if currentToken.get_type() == TokensTypes::LET_VAR
                && tokens[i + 3].get_type() != TokensTypes::INT_TYPE
                && tokens[i + 3].get_type() != TokensTypes::FLOAT_TYPE
                && tokens[i + 3].get_type() != TokensTypes::STRING_TYPE
            {
                err.push(LexerError::new(
                    "error in variable declaration. [let example:int = 4]".to_string(),
                    format!("{} :(", currentToken.get_value()),
                ));
                i += 5;
            } else if currentToken.get_type() == TokensTypes::LET_VAR
                && tokens[i + 4].get_type() != TokensTypes::EQUAL
            {
                err.push(LexerError::new(
                    "error in variable declaration. [let example:int = 4]".to_string(),
                    format!("{} :(", currentToken.get_value()),
                ));
                i += 5;
            } else if currentToken.get_type() == TokensTypes::LET_VAR
                && tokens[i + 5].get_type() != TokensTypes::INT
                && tokens[i + 5].get_type() != TokensTypes::FLOAT
                && tokens[i + 5].get_type() != TokensTypes::STRING
            {
                err.push(LexerError::new(
                    "error in variable declaration. [let example:int = 4]".to_string(),
                    format!("{} :(", currentToken.get_value()),
                ));
                i += 5;
            } else if currentToken.get_type() == TokensTypes::LET_VAR
                && tokens[i + 3].get_type() == TokensTypes::INT_TYPE
                && tokens[i + 5].get_type() == TokensTypes::FLOAT
            {
                err.push(LexerError::new(
                    "TYPE ERROR [let example:int = 4]".to_string(),
                    format!("{} :(", currentToken.get_value()),
                ));
                i += 5;
            } else if currentToken.get_type() == TokensTypes::LET_VAR
                && tokens[i + 3].get_type() == TokensTypes::FLOAT_TYPE
                && tokens[i + 5].get_type() == TokensTypes::INT
            {
                err.push(LexerError::new(
                    "TYPE ERROR [let example:int = 4]".to_string(),
                    format!("{} :(", currentToken.get_value()),
                ));
                i += 5;
            } else if currentToken.get_type() == TokensTypes::QUOTE {
                err.push(LexerError::new(
                    "STRING not terminate in \"".to_string(),
                    format!("{} :(", currentToken.get_value()),
                ));
            }
            i += 1;
        }
        let mut numIf: usize = 0;
        let mut numIfEnds: usize = 0;
        let mut klo: usize = 0;
        while klo.clone() < tokens.len() {
            if tokens[klo].get_type() == TokensTypes::IF_CONDITION {
                numIf += 1;
            } else if tokens[klo].get_type() == TokensTypes::END_IF {
                numIfEnds += 1;
            }
            klo += 1;
        }
        if numIf != numIfEnds {
            err.push(LexerError::new(
                "all if lenght != all end-if lenght".to_string(),
                "if".to_string(),
            ));
        }

        let mut numWhile: usize = 0;
        let mut numWhileEnds: usize = 0;
        let mut klo: usize = 0;
        while klo.clone() < tokens.len() {
            if tokens[klo].get_type() == TokensTypes::WHILE_LOOP {
                numWhile += 1;
            } else if tokens[klo].get_type() == TokensTypes::END_WHILE_LOOP {
                numWhileEnds += 1;
            }
            klo += 1;
        }
        if numWhile != numWhileEnds {
            err.push(LexerError::new(
                "all while lenght != all end-while lenght".to_string(),
                "while".to_string(),
            ));
        }

        err
    }

    pub fn is_valid_new_text(&mut self) -> bool {
        let mut last_text: String = "".to_string();
        let mut valid: bool = true;
        if self.position != 0 {
            last_text = self
                .text
                .chars()
                .nth(self.position - 1)
                .unwrap()
                .to_string();
            if last_text == ":" {
                valid = true;
            } else if last_text == "+"
                || last_text == "-"
                || last_text == "*"
                || last_text == "/"
                || last_text == " "
                || last_text == ""
                || last_text == "\n"
                || last_text == "("
                || last_text == ")"
            {
                valid = true;
            } else {
                valid = false;
            }
        } else {
            last_text = String::from(" ");
            valid = true;
        }
        return valid == true && last_text != "";
    }
    pub fn run_laxer(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.is_final() == false {
            if self.get_current_char() == "+" {
                tokens.push(Token::new("+".to_string(), TokensTypes::PLUS));
            } else if self.position + 1 < self.text.len()
                && self.get_current_char() == "-"
                && self
                    .text
                    .chars()
                    .nth(self.position + 1)
                    .unwrap()
                    .to_string()
                    == ">"
            {
                tokens.push(Token::new("->".to_string(), TokensTypes::RETURN_SYMBOL));
            } else if self.get_current_char() == "," {
                tokens.push(Token::new(",".to_string(), TokensTypes::COMMA));
            } else if self.get_current_char() == "\"" {
                let mut iko: usize = self.position + 1;
                let mut h_text: String = "".to_string();
                let mut ter: bool = false;

                while iko.clone() < self.text.len().clone()
                    && self.text.chars().nth(iko).unwrap().to_string() != "\""
                {
                    h_text.push(self.text.chars().nth(iko).unwrap());
                    iko += 1;
                    if iko < self.text.len() {
                        if self.text.chars().nth(iko).unwrap().to_string() == "\"" {
                            ter = true;
                            tokens.push(Token::new(h_text.to_string(), TokensTypes::STRING));
                        }
                        self.position = iko;
                    }
                }
                if ter == false {
                    tokens.push(Token::new("\"".to_string(), TokensTypes::QUOTE));
                }
            } else if self.get_current_char() == "-" {
                tokens.push(Token::new("-".to_string(), TokensTypes::MINUS));
            } else if self.get_current_char() == "\n" {
                tokens.push(Token::new("\n".to_string(), TokensTypes::END_LINE));
            } else if self.get_current_char() == "*" {
                tokens.push(Token::new("*".to_string(), TokensTypes::MULT));
            } else if self.get_current_char() == "/" {
                tokens.push(Token::new("/".to_string(), TokensTypes::DIV));
            } else if self.get_current_char() == "(" {
                tokens.push(Token::new("(".to_string(), TokensTypes::L_PARENT));
            } else if self.get_current_char() == ")" {
                tokens.push(Token::new(")".to_string(), TokensTypes::R_PARENT));
            } else if self.get_current_char() == "." {
                tokens.push(Token::new(".".to_string(), TokensTypes::POINT));
            } else if self.get_current_char() == " " {
                tokens.push(Token::new(" ".to_string(), TokensTypes::SPACE));
            } else if self.get_current_char() == "<"
                && self.position + 1 < self.text.len()
                && self
                    .text
                    .chars()
                    .nth(self.position + 1)
                    .unwrap()
                    .to_string()
                    == "="
            {
                tokens.push(Token::new("<=".to_string(), TokensTypes::IF_IS_LESS_EQUALS));
                self.position += 1;
            } else if self.get_current_char() == "="
                && self.position + 1 < self.text.len()
                && self
                    .text
                    .chars()
                    .nth(self.position + 1)
                    .unwrap()
                    .to_string()
                    == "="
            {
                tokens.push(Token::new("==".to_string(), TokensTypes::IF_IS_EQUALS));
                self.position += 1;
            } else if self.get_current_char() == ">"
                && self.position + 1 < self.text.len()
                && self
                    .text
                    .chars()
                    .nth(self.position + 1)
                    .unwrap()
                    .to_string()
                    == "="
            {
                tokens.push(Token::new(
                    ">=".to_string(),
                    TokensTypes::IF_IS_GREATER_EQUALS,
                ));
                self.position += 1;
            } else if self.get_current_char() == "<" {
                tokens.push(Token::new("<".to_string(), TokensTypes::IF_IS_LESS))
            } else if self.get_current_char() == ">" {
                tokens.push(Token::new(">".to_string(), TokensTypes::IF_IS_GREATER))
            } else if self.get_current_char() == "=" {
                tokens.push(Token::new("=".to_string(), TokensTypes::EQUAL));
            } else if self.is_var_definition() && self.is_valid_new_text() {
                tokens.push(Token::new("let".to_string(), TokensTypes::LET_VAR));
                self.position += 2;
            } else if self.is_number_current_char() {
                //numebers

                tokens.push(self.create_numbers());
            } else if self.get_current_char() == ":" {
                tokens.push(Token::new(":".to_string(), TokensTypes::TWO_POINTS));
            } else if self.position + 1 < self.text.len()
                && self.get_current_char() == "i"
                && self
                    .text
                    .chars()
                    .nth(self.position + 1)
                    .unwrap()
                    .to_string()
                    == "f"
                && self.is_valid_new_text()
            {
                tokens.push(Token::new("if".to_string(), TokensTypes::IF_CONDITION));
                self.position += 1;
            } else if self.position + 5 < self.text.len()
                && self.get_current_char() == "e"
                && self
                    .text
                    .chars()
                    .nth(self.position + 1)
                    .unwrap()
                    .to_string()
                    == "n"
                && self
                    .text
                    .chars()
                    .nth(self.position + 2)
                    .unwrap()
                    .to_string()
                    == "d"
                && self
                    .text
                    .chars()
                    .nth(self.position + 3)
                    .unwrap()
                    .to_string()
                    == "-"
                && self
                    .text
                    .chars()
                    .nth(self.position + 4)
                    .unwrap()
                    .to_string()
                    == "i"
                && self
                    .text
                    .chars()
                    .nth(self.position + 5)
                    .unwrap()
                    .to_string()
                    == "f"
                && self.is_valid_new_text()
            {
                tokens.push(Token::new("end-if".to_string(), TokensTypes::END_IF));
                self.position += 5;
            } else if self.position + 2 < self.text.len()
                && self.get_current_char() == "f"
                && self
                    .text
                    .chars()
                    .nth(self.position + 1)
                    .unwrap()
                    .to_string()
                    == "u"
                && self
                    .text
                    .chars()
                    .nth(self.position + 2)
                    .unwrap()
                    .to_string()
                    == "n"
                && self.is_valid_new_text()
            {
                tokens.push(Token::new("fun".to_string(), TokensTypes::FUNCTION));
                self.position += 2;
            } else if self.position + 2 < self.text.len()
                && self.get_current_char() == "n"
                && self
                    .text
                    .chars()
                    .nth(self.position + 1)
                    .unwrap()
                    .to_string()
                    == "o"
                && self
                    .text
                    .chars()
                    .nth(self.position + 2)
                    .unwrap()
                    .to_string()
                    == "t"
                && self.is_valid_new_text()
            {
                tokens.push(Token::new("not".to_string(), TokensTypes::NOT));
                self.position += 2;
            } else if self.position + 2 < self.text.len()
                && self.get_current_char() == "i"
                && self
                    .text
                    .chars()
                    .nth(self.position + 1)
                    .unwrap()
                    .to_string()
                    == "n"
                && self
                    .text
                    .chars()
                    .nth(self.position + 2)
                    .unwrap()
                    .to_string()
                    == "t"
                && self.is_valid_new_text()
            {
                tokens.push(Token::new("int_type".to_string(), TokensTypes::INT_TYPE));
                self.position += 2;
            } else if self.position + 5 < self.text.len()
                && self.get_current_char() == "s"
                && self
                    .text
                    .chars()
                    .nth(self.position + 1)
                    .unwrap()
                    .to_string()
                    == "t"
                && self
                    .text
                    .chars()
                    .nth(self.position + 2)
                    .unwrap()
                    .to_string()
                    == "r"
                && self
                    .text
                    .chars()
                    .nth(self.position + 3)
                    .unwrap()
                    .to_string()
                    == "i"
                && self
                    .text
                    .chars()
                    .nth(self.position + 4)
                    .unwrap()
                    .to_string()
                    == "n"
                && self
                    .text
                    .chars()
                    .nth(self.position + 5)
                    .unwrap()
                    .to_string()
                    == "g"
                && self.is_valid_new_text()
            {
                tokens.push(Token::new(
                    "string_type".to_string(),
                    TokensTypes::STRING_TYPE,
                ));
                self.position += 5; //string
            } else if self.position + 4 < self.text.len()
                && self.get_current_char() == "f"
                && self
                    .text
                    .chars()
                    .nth(self.position + 1)
                    .unwrap()
                    .to_string()
                    == "l"
                && self
                    .text
                    .chars()
                    .nth(self.position + 2)
                    .unwrap()
                    .to_string()
                    == "o"
                && self
                    .text
                    .chars()
                    .nth(self.position + 3)
                    .unwrap()
                    .to_string()
                    == "a"
                && self
                    .text
                    .chars()
                    .nth(self.position + 4)
                    .unwrap()
                    .to_string()
                    == "t"
                && self.is_valid_new_text()
            {
                tokens.push(Token::new(
                    "float_type".to_string(),
                    TokensTypes::FLOAT_TYPE,
                ));
                self.position += 4;
            } else if self.position + 4 < self.text.len()
                && self.get_current_char() == "w"
                && self
                    .text
                    .chars()
                    .nth(self.position + 1)
                    .unwrap()
                    .to_string()
                    == "h"
                && self
                    .text
                    .chars()
                    .nth(self.position + 2)
                    .unwrap()
                    .to_string()
                    == "i"
                && self
                    .text
                    .chars()
                    .nth(self.position + 3)
                    .unwrap()
                    .to_string()
                    == "l"
                && self
                    .text
                    .chars()
                    .nth(self.position + 4)
                    .unwrap()
                    .to_string()
                    == "e"
                && self.is_valid_new_text()
            {
                tokens.push(Token::new("while".to_string(), TokensTypes::WHILE_LOOP));
                self.position += 4;
            } else if self.position + 8 < self.text.len()
                && self.get_current_char() == "e"
                && self
                    .text
                    .chars()
                    .nth(self.position + 1)
                    .unwrap()
                    .to_string()
                    == "n"
                && self
                    .text
                    .chars()
                    .nth(self.position + 2)
                    .unwrap()
                    .to_string()
                    == "d"
                && self
                    .text
                    .chars()
                    .nth(self.position + 3)
                    .unwrap()
                    .to_string()
                    == "-"
                && self
                    .text
                    .chars()
                    .nth(self.position + 4)
                    .unwrap()
                    .to_string()
                    == "w"
                && self
                    .text
                    .chars()
                    .nth(self.position + 5)
                    .unwrap()
                    .to_string()
                    == "h"
                && self
                    .text
                    .chars()
                    .nth(self.position + 6)
                    .unwrap()
                    .to_string()
                    == "i"
                && self
                    .text
                    .chars()
                    .nth(self.position + 7)
                    .unwrap()
                    .to_string()
                    == "l"
                && self
                    .text
                    .chars()
                    .nth(self.position + 8)
                    .unwrap()
                    .to_string()
                    == "e"
                && self.is_valid_new_text()
            {
                tokens.push(Token::new(
                    "end-while".to_string(),
                    TokensTypes::END_WHILE_LOOP,
                ));
                self.position += 8;
            } else {
                tokens.push(Token::new(self.get_current_char(), TokensTypes::TEXT));
            }

            self.next_position();
        }
        tokens.clone()
    }
}
