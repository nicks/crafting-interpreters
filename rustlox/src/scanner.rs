// Purpose: Scanner for the Lox language.

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: i32,
}

#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    
    // Literals.
    Identifier, String, Number,
    
    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
    
    Error, EOF,
}

impl Default for TokenType {
    fn default() -> Self { TokenType::EOF }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub start: *const u8,
    pub length: usize,
    pub line: i32,
}

static EMPTY_STRING: &str = "";

impl Default for Token {
    fn default() -> Self {
        return Token{
            token_type: TokenType::EOF,
            start: EMPTY_STRING.as_ptr(),
            length: 0,
            line: 0,
        }
    }
}

impl Token {
    pub fn text(&self) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.start, self.length);
            return std::str::from_utf8(slice).unwrap();
        }
    }
}

pub fn new_scanner(source: String) -> Scanner {
    return Scanner{
        source: source,
        current: 0,
        start: 0,
        line: 1,
    }
}

const UNEXPECTED_CHAR: &str = "Unexpected character.";

impl Scanner {
    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        
        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c = self.advance();
        if self.is_alpha(c) {
            return self.identifier();
        }
        if self.is_digit(c) {
            return self.number();
        }
        return match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::BangEqual);
                } 
                return self.make_token(TokenType::Bang);
            },
            '=' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::EqualEqual);
                }
                return self.make_token(TokenType::Equal);
            },
            '<' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::LessEqual);
                }
                return self.make_token(TokenType::Less);
            },
            '>' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::GreaterEqual);
                }
                return self.make_token(TokenType::Greater);
            },
            '"' => self.string(),
            _ => self.error_token(UNEXPECTED_CHAR),
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        self.advance();
        return self.make_token(TokenType::String);
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') ||
               (c >= 'A' && c <= 'Z') ||
                c == '_';
    }

    fn identifier(&mut self) -> Token {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }
        return self.make_token(self.identifier_type());
    }

    fn identifier_type(&self) -> TokenType {
        return match self.source.as_bytes()[self.start] as char {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            'f' => {
                if self.current - self.start <= 1 {
                    return TokenType::Identifier;
                }
                return match self.source.as_bytes()[self.start + 1] as char {
                    'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                    'o' => self.check_keyword(2, 1, "r", TokenType::For),
                    'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                    _ => TokenType::Identifier,
                }
            },
            't' => {
                if self.current - self.start <= 1 {
                    return TokenType::Identifier;
                }
                return match self.source.as_bytes()[self.start + 1] as char {
                    'h' => self.check_keyword(2, 2, "is", TokenType::This),
                    'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                    _ => TokenType::Identifier,
                }
            },
            _ => TokenType::Identifier,
        }
    }

    fn check_keyword(&self, start: usize, length: usize, rest: &str, token_type: TokenType) -> TokenType {
        if (self.current - self.start == start + length) &&
            (&self.source[self.start + start..self.start + start + length] == rest) {
            return token_type;
        }
        return TokenType::Identifier;
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn number(&mut self) -> Token {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        return self.make_token(TokenType::Number);
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.source.as_bytes()[self.current - 1] as char;
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                },
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                },
                _ => return,
            }
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current] as char != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.as_bytes()[self.current] as char;
    }

    fn peek_next(&self) -> char {
        if  self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.as_bytes()[self.current + 1] as char;
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let slice = &self.source[self.start..self.current];
        return Token{
            token_type: token_type,
            start: slice.as_ptr(),
            length: slice.len(),
            line: self.line,
        }
    }

    fn error_token(&self, message: &str) -> Token {
        return Token{
            token_type: TokenType::Error,
            start: message.as_ptr(),
            length: message.len(),
            line: self.line,
        }
    }
}

