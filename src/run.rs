use crate::lexer::{Token, TYPES};
use crate::read_file;
use crate::stack::Stack;

struct Interpreter {
    stack: Stack<isize>,
    vars: Vec<(String, isize)>,
    macros: Vec<(String, Vec<Token>)>
} impl Interpreter {
    pub fn new() -> Self { Self { stack: Stack::new(), vars: Vec::new(), macros: Vec::new() } }
    pub fn interpret(&mut self, tokens: &Vec<Token>) -> Result<(), String> {
        for token in tokens {
            match &token.token {
                TYPES::INT(value) => self.stack.push(*value),
                TYPES::BODY(tokens_) => self.interpret(tokens_).unwrap(),
                TYPES::SET(id) => {
                    let mut found = false;
                    for i in 0..self.vars.len() {
                        if self.vars[i].0 == id.clone() {
                            found = true;
                            if self.stack.len() < 1 {
                                self.vars[i].1 = 0;
                            } else {
                                self.vars[i].1 = self.stack.pop().unwrap();
                            }
                            break
                        }
                    }
                    if !found { self.vars.push((id.clone(), self.stack.pop().unwrap())); }
                }
                TYPES::ID(id) => {
                    let mut found = false;
                    for i in 0..self.vars.len() {
                        if self.vars[i].0 == id.clone() {
                            found = true;
                            self.stack.push(self.vars[i].1);
                            break
                        }
                    }
                    if !found { return Err(String::from(format!("ID ERROR: id '{}' not registered", id))) }
                }
                TYPES::REPEAT(tokens_) => {
                    if self.stack.len() < 1 { continue }
                    let a = self.stack.pop().unwrap();
                    for i in 0..a { self.interpret(tokens_).unwrap(); }
                }
                TYPES::WHILE(tokens_) => {
                    if self.stack.len() < 1 { continue }
                    while self.stack.peek().unwrap() != &0 { self.interpret(tokens_).unwrap(); }
                }
                TYPES::IF(tokens_) => {
                    if self.stack.len() < 1 { continue }
                    let a = self.stack.peek().unwrap();
                    if a != &0 { self.stack.pop(); self.interpret(tokens_).unwrap(); }
                }
                TYPES::ADD => {
                    if self.stack.len() < 2 { continue }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b)
                }
                TYPES::SUB => {
                    if self.stack.len() < 2 { continue }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b)
                }
                TYPES::MUL => {
                    if self.stack.len() < 2 { continue }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b)
                }
                TYPES::DIV => {
                    if self.stack.len() < 2 { continue }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b)
                }
                TYPES::EQ => {
                    if self.stack.len() < 2 { continue }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a == b) as isize)
                }
                TYPES::NE => {
                    if self.stack.len() < 2 { continue }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a != b) as isize)
                }
                TYPES::LT => {
                    if self.stack.len() < 2 { continue }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a < b) as isize)
                }
                TYPES::GT => {
                    if self.stack.len() < 2 { continue }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a > b) as isize)
                }
                TYPES::NOT => {
                    if self.stack.len() < 1 { continue }
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a == 0) as isize)
                }
                _ => println!("unknown token {:?}", token.token)
            }
        }
        Ok(())
    }
}

pub fn run(tokens: Vec<Token>) -> Result<Stack<isize>, String> {
    let mut interpreter = Interpreter::new();
    let res = interpreter.interpret(&tokens);
    if res.is_err() { return Err(res.err().unwrap()) }
    Ok(interpreter.stack)
}