use crate::scanner::new_scanner;
use crate::scanner::TokenType;

pub fn compile(source: String) {
    let mut scanner = new_scanner(source);
    let mut line = std::usize::MAX;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:04} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:2} '{}'", token.token_type as u8, token.text);
        if token.token_type == TokenType::EOF || token.token_type == TokenType::Error {
            break;
        }
    }
}
