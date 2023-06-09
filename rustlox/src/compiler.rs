use crate::scanner::new_scanner;
use crate::scanner::Token;
use crate::scanner::TokenType;
use crate::scanner::Scanner;
use crate::value::Value;
use crate::chunk::Chunk;
use crate::debug::disassemble_chunk;
use crate::chunk::OpCode;
use crate::object::Obj;
use crate::object::ObjArray;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;

const DEBUG: bool = false;

struct Parser<'a> {
    rules: [ParseRule; TOKEN_COUNT],
    scanner: Scanner,
    obj_array: &'a mut ObjArray,
    chunk: &'a mut Chunk,
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum Precedence {
    None,
    Assignment,  // =
    Or,          // or
    And,         // and
    Equality,    // == !=
    Comparison,  // < > <= >=
    Term,        // + -
    Factor,      // * /
    Unary,       // ! -
    Call,        // . ()
    Primary,
}

struct ParseRule {
    prefix: Option<fn(&mut Parser)>,
    infix: Option<fn(&mut Parser)>,
    precedence: Precedence,
}

impl ParseRule {
    fn new(prefix: Option<fn(&mut Parser)>, infix: Option<fn(&mut Parser)>, precedence: Precedence) -> ParseRule {
        ParseRule {
            prefix: prefix,
            infix: infix,
            precedence: precedence,
        }
    }
}

const TOKEN_COUNT: usize = 40;
const NONE_RULE: ParseRule = ParseRule{
    prefix: None,
    infix: None,
    precedence: Precedence::None,
};
fn rules_table() -> [ParseRule; TOKEN_COUNT] {
    let mut table = [NONE_RULE; TOKEN_COUNT];
    table[TokenType::LeftParen as usize] =
        ParseRule::new(Some(grouping), None, Precedence::None);
    table[TokenType::RightParen as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::LeftBrace as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::RightBrace as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::Comma as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::Dot as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::Minus as usize] =
        ParseRule::new(Some(unary), Some(binary), Precedence::Term);
    table[TokenType::Plus as usize] =
        ParseRule::new(None, Some(binary), Precedence::Term);
    table[TokenType::Semicolon as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::Slash as usize] =
        ParseRule::new(None, Some(binary), Precedence::Factor);
    table[TokenType::Star as usize] =
        ParseRule::new(None, Some(binary), Precedence::Factor);
    table[TokenType::Bang as usize] =
        ParseRule::new(Some(unary), None, Precedence::None);
    table[TokenType::BangEqual as usize] =
        ParseRule::new(None, Some(binary), Precedence::Equality);
    table[TokenType::Equal as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::EqualEqual as usize] =
        ParseRule::new(None, Some(binary), Precedence::Equality);
    table[TokenType::Greater as usize] =
        ParseRule::new(None, Some(binary), Precedence::Comparison);
    table[TokenType::GreaterEqual as usize] =
        ParseRule::new(None, Some(binary), Precedence::Comparison);
    table[TokenType::Less as usize] =
        ParseRule::new(None, Some(binary), Precedence::Comparison);
    table[TokenType::LessEqual as usize] =
        ParseRule::new(None, Some(binary), Precedence::Comparison);
    table[TokenType::Identifier as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::String as usize] =
        ParseRule::new(Some(string), None, Precedence::None);
    table[TokenType::Number as usize] =
        ParseRule::new(Some(number), None, Precedence::None);
    table[TokenType::And as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::Class as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::Else as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::False as usize] =
        ParseRule::new(Some(literal), None, Precedence::None);
    table[TokenType::For as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::Fun as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::If as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::Nil as usize] =
        ParseRule::new(Some(literal), None, Precedence::None);
    table[TokenType::Or as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::Print as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::Return as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::Super as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::This as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::True as usize] =
        ParseRule::new(Some(literal), None, Precedence::None);
    table[TokenType::Var as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::While as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::Error as usize] =
        ParseRule::new(None, None, Precedence::None);
    table[TokenType::EOF as usize] =
        ParseRule::new(None, None, Precedence::None);
    return table;
}

pub fn compile(source: String, chunk: &mut Chunk, obj_array: &mut ObjArray) -> bool {
    let mut parser = Parser{
        rules: rules_table(),
        scanner: new_scanner(source),
        chunk: chunk,
        obj_array: obj_array,
        current: Token::default(),
        previous: Token::default(),
        had_error: false,
        panic_mode: false,
    };
    parser.advance();
    parser.expression();
    parser.consume(TokenType::EOF, "Expect end of expression.");
    parser.end_compiler();
    return !parser.had_error;
}

impl Parser<'_> {
    fn advance(&mut self) {
        self.previous = std::mem::take(&mut self.current);
        loop {
            self.current = self.scanner.scan_token();
            if self.current.token_type != TokenType::Error {
                break;
            }
            self.error_at_current("");
        }
    }

    fn error_at_current(&mut self, message: &str) {
        let token = std::mem::take(&mut self.current);
        self.error_at(&token, message);
        self.current = token;
    }

    fn error(&mut self, message: &str) {
        let token = std::mem::take(&mut self.previous);
        self.error_at(&token, message);
        self.previous = token;
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        
        eprint!("[line {}] Error", token.line);
        if token.token_type == TokenType::EOF {
            eprint!(" at end");
        } else if token.token_type == TokenType::Error {
            // Nothing.
        } else {
            eprint!(" at '{}'", token.text());
        }
        if message != "" {
            eprint!(": {}", message);
        }
        eprintln!();
        self.had_error = true;
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_chunk(byte, self.previous.line);
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.chunk
    }

    fn end_compiler(&mut self) {
        self.emit_return();
        if DEBUG && !self.had_error {
            disassemble_chunk(self.current_chunk(), "code");
        }
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant as u8, constant);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let chunk = self.current_chunk();
        let constant = chunk.add_constant(value);
        if constant > usize::MAX {
            self.error("Too many constants in one chunk.");
            return 0;
        }
        return constant as u8;
    }
    
    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self.rules[self.previous.token_type as usize].prefix;
        if prefix_rule.is_none() {
            self.error("Expect expression.");
            return;
        }
        prefix_rule.unwrap()(self);

        while precedence <= self.get_rule(self.current.token_type).precedence {
            self.advance();
            let infix_rule = self.get_rule(self.previous.token_type).infix;
            if infix_rule.is_none() {
                self.error("Expect expression.");
                return;
            }
            infix_rule.unwrap()(self);
        }
    }

    fn get_rule(&self, token_type: TokenType) -> &ParseRule {
        &self.rules[token_type as usize]
    }
}

fn grouping(parser: &mut Parser) {
    parser.expression();
    parser.consume(TokenType::RightParen, "Expect ')' after expression.");
}

fn number(parser: &mut Parser) {
    let value = parser.previous.text().parse::<f64>().unwrap();
    parser.emit_constant(Value::number(value));
}

fn string(parser: &mut Parser) {
    let text = parser.previous.text();
    let value = parser.obj_array.copy_string(&text[1..text.len() - 1]);
    parser.emit_constant(Value::object(value as *const Obj));
}

fn literal(parser: &mut Parser) {
    match parser.previous.token_type {
        TokenType::False => parser.emit_byte(OpCode::False.into()),
        TokenType::Nil => parser.emit_byte(OpCode::Nil.into()),
        TokenType::True => parser.emit_byte(OpCode::True.into()),
        _ => unreachable!(),
    }
}

fn unary(parser: &mut Parser) {
    let operator_type = parser.previous.token_type;
    parser.parse_precedence(Precedence::Unary);
    
    match operator_type {
        TokenType::Minus => parser.emit_byte(OpCode::Negate as u8),
        TokenType::Bang => parser.emit_byte(OpCode::Not as u8),
        _ => unreachable!(),
    }
}

fn binary(parser: &mut Parser) {
    let operator_type = parser.previous.token_type;
    let rule = parser.get_rule(operator_type);

    let p: u8 = rule.precedence.into();
    parser.parse_precedence(
        Precedence::try_from(p + 1).unwrap());
    
    match operator_type {
        TokenType::Plus => parser.emit_byte(OpCode::Add.into()),
        TokenType::Minus => parser.emit_byte(OpCode::Subtract.into()),
        TokenType::Star => parser.emit_byte(OpCode::Multiply.into()),
        TokenType::Slash => parser.emit_byte(OpCode::Divide.into()),
        TokenType::BangEqual => {
            parser.emit_bytes(OpCode::Equal.into(), OpCode::Not.into());
        },
        TokenType::EqualEqual => parser.emit_byte(OpCode::Equal.into()),
        TokenType::Greater => parser.emit_byte(OpCode::Greater.into()),
        TokenType::GreaterEqual => {
            parser.emit_bytes(OpCode::Less.into(), OpCode::Not.into());
        },
        TokenType::Less => parser.emit_byte(OpCode::Less.into()),
        TokenType::LessEqual => {
            parser.emit_bytes(OpCode::Greater.into(), OpCode::Not.into());
        },
        _ => unreachable!(),
    }
}

