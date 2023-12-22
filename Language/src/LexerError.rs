#[derive(Clone, Debug)]
pub struct LexerError {
    charError: String,
    messageError: String,
}
impl LexerError {
    pub fn new(messageError: String, charError: String) -> Self {
        LexerError {
            messageError: messageError,
            charError: charError,
        }
    }
    pub fn show_error(&self) {
        println!(
            "lexer error in: `{}` @message: '{}' ",
            self.charError, self.messageError
        )
    }
}
