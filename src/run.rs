use crate::lexer::{Token, TYPES};
use crate::read_file;
use crate::stack::Stack;
use std::collections::HashMap;

struct Interpreter {
    stack: Stack<isize>,
    vars: HashMap<String, isize>,
    memory: Vec<Vec<Token>>,
    macros: HashMap<String, usize>
} impl Interpreter {
    pub fn new() -> Self {
        Self { stack: Stack::new(), vars: HashMap::new(), macros: HashMap::new(), memory: Vec::new() }
    }
    pub fn interpret(&mut self, tokens: &Vec<Token>) -> Result<(), String> {
        for token in tokens {
            match &token.token {
                TYPES::INT(value) => self.stack.push(*value),
                TYPES::BODY(tokens_) => self.interpret(tokens_).unwrap(),
                TYPES::SET(id) => {
                    if self.macros.contains_key(id) {
                        return Err(String::from("ID ERROR: id is registered macro and cannot be redefined"))
                    }
                    if self.vars.contains_key(id) {
                        self.vars.insert(id.clone(), self.stack.pop().unwrap());
                        continue
                    }
                    self.vars.insert(id.clone(), self.stack.pop().unwrap());
                }
                TYPES::MACRO(id, tokens_) => {
                    if self.macros.contains_key(id) {
                        return Err(String::from(format!("MACRO ERROR: macro '{id}' is already defined")))
                    }
                    self.memory.push(tokens_.clone());
                    self.macros.insert(id.clone(), self.memory.len()-1);
                }
                TYPES::ID(id) => {
                    if self.vars.contains_key(id) {
                        self.stack.push(self.vars.get(id).unwrap().clone());
                        continue
                    } else if self.macros.contains_key(id) {
                        let ptr = self.macros.get(id).unwrap().clone();
                        let tokens_ = self.memory[ptr].clone();
                        self.interpret(&tokens_).unwrap();
                        continue
                    }
                    return Err(String::from(format!("ID ERROR: id '{}' not registered", id)))
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