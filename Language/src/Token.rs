#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokensTypes {
    PLUS,
    INT,
    FLOAT,
    MINUS,
    MULT,
    DIV,
    SINTAX_ERROR,
    POINT,
    R_PARENT,
    L_PARENT,
    SPACE,
    TEXT, // text is not a string example: 30 * h   is errr
    LET_VAR,
    EQUAL,
    TWO_POINTS,
    INT_TYPE,
    FLOAT_TYPE,
    END_LINE,
    QUOTE, //comillas
    STRING,
    STRING_TYPE,
    BRACKET_START,
    BRACKET_END,
    FUNCTION,
    COMMA,

    RETURN_SYMBOL,
}
#[derive(Clone, Debug)]

pub struct Token {
    value: String,
    token_type: TokensTypes,
}

impl Token {
    pub fn new(value: String, token_type: TokensTypes) -> Self {
        Token {
            value: value,
            token_type: token_type,
        }
    }

    pub fn get_type(&self) -> TokensTypes {
        self.token_type.clone()
    }
    pub fn get_value(&self) -> String {
        return self.value.clone();
    }
    pub fn ignore_space_tokens(tokens: Vec<Token>) -> Vec<Token> {
        let mut n_t: Vec<Token> = Vec::new();
        let mut i: usize = 0;
        while i < tokens.len() {
            if tokens[i].get_type() != TokensTypes::SPACE {
                n_t.push(tokens[i].clone());
            }
            i += 1;
        }
        n_t
    }
}
