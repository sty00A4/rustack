#![allow(dead_code)]
#![allow(unused)]

// GLOBALS
static DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
static LETTERS: [&str; 53] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
    "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
    "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "_"
];

// FILE
#[derive(Debug)]
pub struct File {
    pub name: String,
    pub text: String
}

// TOKEN
#[derive(Debug, PartialEq)]
pub enum TYPES {
    INT(isize), TYPE, BODY(Vec<Token>),
    ADD, SUB, MUL, DIV, EQ, NE, LT, GT, NOT,
    IF(Vec<Token>), REPEAT(Vec<Token>), WHILE(Vec<Token>),
    SET(String), ID(String),
}
#[derive(Debug, PartialEq)]
pub struct Token {
    pub token: TYPES,
    pub start: usize,
    pub stop: usize
} impl Token {
    pub fn new(token: TYPES, start: usize, stop: usize) -> Self { Self { token, start, stop } }
}

// LEXER
#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    file: File,
    idx: usize
} impl Lexer {
    pub fn new(mut file: File) -> Self { Self { tokens: Vec::new(), file, idx: 0 } }
    pub fn advance(&mut self) { self.idx += 1 }
    pub fn char(&self) -> &str {
        if self.idx >= self.file.text.len() { return "" }
        &self.file.text[self.idx..self.idx+1]
    }
    pub fn range(&self, start: usize, stop: usize) -> &str { &self.file.text[start..stop] }
    pub fn next(&mut self) -> Result<Token, String> {
        while self.char() == " " || self.char() == "\t" || self.char() == "\n" { self.advance(); }
        let start = self.idx;
        // NUMBER
        if DIGITS.contains(&self.char()) {
            while DIGITS.contains(&self.char()) { self.advance(); }
            let number = self.range(start, self.idx);
            return Ok(Token::new(TYPES::INT(number.parse::<isize>().unwrap()), start, self.idx));
        }
        // BODY
        if self.char() == "(" {
            self.advance();
            let mut tokens: Vec<Token> = Vec::new();
            while self.char() != ")" && self.char() != "" {
                let res = self.next();
                if res.is_err() { return Err(res.err().unwrap()) }
                tokens.push(res.unwrap());
            }
            self.advance();
            return Ok(Token::new(TYPES::BODY(tokens), start, self.idx));
        }
        // SET
        if self.char() == "@" {
            self.advance();
            if LETTERS.contains(&self.char()) {
                self.advance();
                while LETTERS.contains(&self.char()) || DIGITS.contains(&self.char()) { self.advance(); }
                let word = self.range(start+1, self.idx);
                if ["if", "repeat", "while"].contains(&word) { return Err(String::from("SYNTAX ERROR: expected id, not keyword")) }
                return Ok(Token::new(TYPES::SET(String::from(word)), start, self.idx));
            }
            return Err(String::from("SYNTAX ERROR: expected id"))
        }
        // OPERATION
        if self.char() == "+" {
            self.advance();
            return Ok(Token::new(TYPES::ADD, start, self.idx));
        }
        if self.char() == "-" {
            self.advance();
            return Ok(Token::new(TYPES::SUB, start, self.idx));
        }
        if self.char() == "*" {
            self.advance();
            return Ok(Token::new(TYPES::MUL, start, self.idx));
        }
        if self.char() == "/" {
            self.advance();
            return Ok(Token::new(TYPES::DIV, start, self.idx));
        }
        if self.char() == "=" {
            self.advance();
            return Ok(Token::new(TYPES::EQ, start, self.idx));
        }
        if self.char() == "<" {
            self.advance();
            return Ok(Token::new(TYPES::LT, start, self.idx));
        }
        if self.char() == ">" {
            self.advance();
            return Ok(Token::new(TYPES::GT, start, self.idx));
        }
        if self.char() == "!" {
            self.advance();
            return Ok(Token::new(TYPES::NOT, start, self.idx));
        }
        // WORD
        if LETTERS.contains(&self.char()) {
            self.advance();
            while LETTERS.contains(&self.char()) || DIGITS.contains(&self.char()) { self.advance(); }
            let word = self.range(start, self.idx);
            match word {
                "if" => return Ok(Token::new(TYPES::IF(vec![self.next().unwrap()]), start, self.idx)),
                "repeat" => return Ok(Token::new(TYPES::REPEAT(vec![self.next().unwrap()]), start, self.idx)),
                "while" => return Ok(Token::new(TYPES::WHILE(vec![self.next().unwrap()]), start, self.idx)),
                _ => {}
            }
            return Ok(Token::new(TYPES::ID(String::from(word)), start, self.idx));
        }
        self.idx = start;
        Err(String::from(format!("CHAR ERROR: '{}'", self.char())))
    }
    pub fn lex(&mut self) -> Result<(), String> {
        while self.char() != "" {
            let res = self.next();
            if res.is_err() { return Err(res.err().unwrap()) }
            self.tokens.push(res.unwrap());
        }
        Ok(())
    }
}
pub fn lex(name: &String, text: String) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(File{name: name.clone(), text});
    let res = lexer.lex();
    if res.is_err() { return Err(res.err().unwrap()) }
    return Ok(lexer.tokens)
}