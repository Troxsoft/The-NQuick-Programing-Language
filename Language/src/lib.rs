pub mod GenerateCode;
pub mod Lexer;
pub mod LexerError;
pub mod Token;
pub fn format_and_clean_code(code: String) -> String {
    let mut i: usize = 0;
    let lines = code.split("\n");
    let mut nli: Vec<String> = Vec::new();
    let mut nli_def: Vec<String> = Vec::new();
    let mut nStr: String = String::new();
    for line in lines {
        nli.push(line.clone().to_string());
    }
    let mut current = "".to_string();
    while i < nli.len() {
        current = nli[i].trim().to_string().clone();
        if current.clone() != "" {
            nli_def.push(current.clone());
        }
        i += 1;
    }
    i = 0;
    while i < nli_def.len() {
        nStr.push_str(format!("{}\n", nli_def[i]).as_str());
        i += 1;
    }
    nStr
}
