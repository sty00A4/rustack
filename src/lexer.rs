#![allow(dead_code)]
#![allow(unused)]

use std::cmp::min;

// GLOBALS
static DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
static LETTERS: [&str; 53] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
    "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
    "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "_"
];
static VARS: [&str; 2] = ["STACK", "LENGTH"];
static WORDS: [&str; 5] = ["if", "repeat", "while", "STACK", "LENGTH"];

// FILE
#[derive(Debug)]
pub struct File {
    pub name: String,
    pub text: String
}

// TOKEN
#[derive(Debug, PartialEq, Clone)]
pub enum TYPES { NONE,
    INT(isize), TYPE, BODY(Vec<Token>), MAP(Vec<String>), SIZE(Box<Token>),
    ADD, SUB, MUL, DIV, EQ, NE, LT, GT, NOT,
    IF(Vec<Token>, Vec<Token>), REPEAT(Vec<Token>), WHILE(Vec<Token>),
    SET(String), ID(String), MACRO(String, Box<Token>, Vec<Token>),
    PRINT,
    VAR(String)
}
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token: TYPES,
    pub start: usize,
    pub stop: usize
} impl Token {
    pub fn new(token: TYPES, start: usize, stop: usize) -> Self { Self { token, start, stop } }
    pub fn none() -> Self { Self { token: TYPES::NONE, start: 0, stop: 0 } }
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
    pub fn range(&self, start: usize, stop: usize) -> &str {
        &self.file.text[min(start, self.file.text.len()-1)..min(stop, self.file.text.len())]
    }
    pub fn word(&mut self) -> Result<&str, String> {
        if !LETTERS.contains(&self.char()) { return Err(String::from("SYNTAX ERROR: expected id")) }
        let start = self.idx;
        while LETTERS.contains(&self.char()) || DIGITS.contains(&self.char()) { self.advance(); }
        Ok(self.range(start, self.idx))
    }
    pub fn next(&mut self) -> Result<Token, String> {
        while self.char() == " " || self.char() == "\t" || self.char() == "\n" { self.advance(); }
        let start = self.idx;
        // NUMBER
        if DIGITS.contains(&self.char()) {
            while DIGITS.contains(&self.char()) { self.advance(); }
            let number = self.range(start, self.idx);
            return Ok(Token::new(TYPES::INT(number.parse::<isize>().unwrap()), start, self.idx));
        }
        // SUBS
        if self.char() == "(" {
            self.advance();
            let mut tokens: Vec<Token> = Vec::new();
            while self.char() != ")" && self.char() != "" {
                let res = self.next();
                if res.is_err() { return Err(res.err().unwrap()) }
                tokens.push(res.unwrap());
                while self.char() == " " || self.char() == "\t" || self.char() == "\n" { self.advance(); }
            }
            self.advance();
            return Ok(Token::new(TYPES::BODY(tokens), start, self.idx));
        }
        if self.char() == "[" {
            self.advance();
            let mut tokens: Vec<Token> = Vec::new();
            let res = self.next();
            if res.is_err() { return Err(res.err().unwrap()) }
            if self.char() != "]" { return Err(res.err().unwrap()) }
            self.advance();
            return Ok(Token::new(TYPES::SIZE(Box::new(res.unwrap())), start, self.idx));
        }
        if self.char() == "{" {
            self.advance();
            let mut ids: Vec<String> = Vec::new();
            while self.char() != "}" && self.char() != "" {
                let res = self.word();
                if res.is_err() { return Err(res.err().unwrap()) }
                let word = res.unwrap();
                ids.push(String::from(word));
                while self.char() == " " || self.char() == "\t" || self.char() == "\n" { self.advance(); }
            }
            self.advance();
            return Ok(Token::new(TYPES::MAP(ids), start, self.idx));
        }
        // SET
        if self.char() == "@" {
            self.advance();
            if LETTERS.contains(&self.char()) {
                let res = self.word();
                if res.is_err() { return Err(res.err().unwrap()) }
                let word = &res.unwrap()[1..];
                if WORDS.contains(&word) { return Err(String::from("SYNTAX ERROR: expected id, not keyword")) }
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
            let res = self.word();
            if res.is_err() { return Err(res.err().unwrap()) }
            let word = res.unwrap();
            match word {
                "if" => {
                    let res = self.next();
                    if res.is_err() { return Err(res.err().unwrap()) }
                    let if_case = res.unwrap();
                    let mut else_case = Token::none();
                    while self.char() == " " || self.char() == "\t" || self.char() == "\n" { self.advance(); }
                    if self.range(self.idx, self.idx+"else".len()) == "else" {
                        self.idx += "else".len();
                        let res = self.next();
                        if res.is_err() { return Err(res.err().unwrap()) }
                        else_case = res.unwrap();
                    }
                    return Ok(Token::new(TYPES::IF(vec![if_case], vec![else_case]), start, self.idx));
                }
                "repeat" => {
                    let res = self.next();
                    if res.is_err() { return Err(res.err().unwrap()) }
                    return Ok(Token::new(TYPES::REPEAT(vec![res.unwrap()]), start, self.idx));
                }
                "while" => {
                    let res = self.next();
                    if res.is_err() { return Err(res.err().unwrap()) }
                    return Ok(Token::new(TYPES::WHILE(vec![res.unwrap()]), start, self.idx));
                }
                "macro" => {
                    if self.char() == "" { return Err(String::from("EOF ERROR: unexpected end of file")) }
                    let mut size = Token::none();
                    if self.char() == "[" {
                        let res = self.next();
                        if res.is_err() { return Err(res.err().unwrap()) }
                        size = res.unwrap();
                    }
                    let res = self.next();
                    if res.is_err() { return Err(res.err().unwrap()) }
                    let mut id_token = res.unwrap();
                    let mut id = String::new();
                    match id_token.token {
                        TYPES::ID(id_) => id = id_,
                        _ => return Err(String::from("SYNTAX ERROR: expected id"))
                    }
                    if self.char() == "" { return Err(String::from("EOF ERROR: unexpected end of file")) }
                    let res = self.next();
                    if res.is_err() { return Err(res.err().unwrap()) }
                    let body = vec![res.unwrap()];
                    return Ok(Token::new(TYPES::MACRO(id, Box::new(size), body), start, self.idx))
                }
                "print" => return Ok(Token::new(TYPES::PRINT, start, self.idx)),
                _ => {}
            }
            if VARS.contains(&word) { return Ok(Token::new(TYPES::VAR(String::from(word)), start, self.idx)) }
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