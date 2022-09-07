use crate::lexer::{lex, Token, TYPES};
use crate::read_file;
use crate::stack::Stack;
use std::collections::HashMap;

struct Interpreter {
    stack: Stack<isize>,
    vars: HashMap<String, isize>,
    memory: Vec<(usize, Vec<Token>)>,
    macros: HashMap<String, usize>
} impl Interpreter {
    pub fn new() -> Self {
        Self { stack: Stack::new(), vars: HashMap::new(), macros: HashMap::new(), memory: Vec::new() }
    }
    pub fn std(&mut self) -> Result<(), String> {
        let path = "std/std.rst";
        let text = read_file(path);
        let res_std_tokens = lex(&String::from(path), text);
        if res_std_tokens.is_err() { return Err(res_std_tokens.err().unwrap()) }
        let std_tokens = res_std_tokens.unwrap();
        let res = self.interpret(&std_tokens);
        if res.is_err() { return Err(res.err().unwrap()) }
        Ok(())
    }
    pub fn get(&mut self, token: Box<Token>) -> Result<isize, String> {
        match token.token {
            TYPES::ID(id) => {
                if self.vars.contains_key(id.as_str()) {
                    return Ok(self.vars.get(id.as_str()).unwrap().clone())
                }
                return Err(String::from(format!("ID ERROR: id '{}' not registered", id)))
            }
            TYPES::VAR(var) => {
                match var.as_str() {
                    "STACK" => { println!("{:?}", self.stack.to_vec()); return Ok(0) },
                    "LENGTH" => return Ok(self.stack.len() as isize),
                    _ => {}
                };
            }
            TYPES::INT(n) => return Ok(n),
            TYPES::SIZE(int) => return self.get(int),
            TYPES::NONE => return Ok(0),
            _ => {}
        }
        Ok(0)
    }
    pub fn interpret(&mut self, tokens: &Vec<Token>) -> Result<(), String> {
        for token in tokens {
            match &token.token {
                TYPES::NONE => {},
                TYPES::INT(value) => self.stack.push(*value),
                TYPES::BODY(tokens_) => {
                    let res = self.interpret(tokens_);
                    if res.is_err() { return Err(res.err().unwrap()) }
                },
                TYPES::SET(id) => {
                    if self.macros.contains_key(id) {
                        return Err(String::from("ID ERROR: id is registered macro and cannot be redefined"))
                    }
                    if self.vars.contains_key(id) {
                        self.vars.insert(id.clone(), self.stack.pop().unwrap());
                        continue
                    }
                    if self.stack.len() == 0 { return Err(String::from("STACK ERROR: nothing on stack to take")) }
                    self.vars.insert(id.clone(), self.stack.pop().unwrap());
                }
                TYPES::MAP(ids) => {
                    if self.stack.len() < ids.len() {
                        return Err(String::from("STACK ERROR: stack size not big enough"))
                    }
                    for id in ids.iter().rev() {
                        if self.macros.contains_key(id) {
                            return Err(String::from("ID ERROR: id is registered macro and cannot be redefined"))
                        }
                        if self.vars.contains_key(id) {
                            self.vars.insert(id.clone(), self.stack.pop().unwrap());
                            continue
                        }
                        self.vars.insert(id.clone(), self.stack.pop().unwrap());
                    }
                }
                TYPES::MACRO(id, size_token, tokens_) => {
                    if self.macros.contains_key(id) {
                        return Err(String::from(format!("MACRO ERROR: macro '{id}' is already defined")))
                    }
                    let res_size = self.get(size_token.clone());
                    if res_size.is_err() { return Err(res_size.err().unwrap()) }
                    let size = res_size.unwrap();
                    self.memory.push((size as usize, tokens_.clone()));
                    self.macros.insert(id.clone(), self.memory.len()-1);
                }
                TYPES::ID(id) => {
                    if self.vars.contains_key(id) {
                        // VAR CALL
                        self.stack.push(self.vars.get(id).unwrap().clone());
                        continue
                    } else if self.macros.contains_key(id) {
                        // MACRO CALL
                        let ptr = self.macros.get(id).unwrap().clone();
                        let (size, tokens_) = self.memory[ptr].clone();
                        if self.stack.len() < size {
                            return Err(String::from(format!("MACRO ERROR: expected at least {size} stack entries")))
                        }
                        let temp = self.vars.clone();
                        self.vars.clear();
                        let res = self.interpret(&tokens_);
                        if res.is_err() { return Err(res.err().unwrap()) }
                        self.vars = temp;
                        continue
                    }
                    return Err(String::from(format!("ID ERROR: id '{}' not registered", id)))
                }
                TYPES::REPEAT(tokens_) => {
                    if self.stack.len() < 1 { continue }
                    let a = self.stack.pop().unwrap();
                    for i in 0..a {
                        let res = self.interpret(tokens_);
                        if res.is_err() { return Err(res.err().unwrap()) }
                    }
                }
                TYPES::WHILE(tokens_) => {
                    if self.stack.len() < 1 { continue }
                    while self.stack.peek().unwrap() != &0 {
                        let res = self.interpret(tokens_);
                        if res.is_err() { return Err(res.err().unwrap()) }
                    }
                }
                TYPES::IF(if_case, else_case) => {
                    if self.stack.len() < 1 { continue }
                    let a = self.stack.peek().unwrap();
                    if a != &0 {
                        self.stack.pop();
                        let res = self.interpret(if_case);
                        if res.is_err() { return Err(res.err().unwrap()) }
                    } else if else_case[0].token != TYPES::NONE {
                        self.stack.pop();
                        let res = self.interpret(else_case);
                        if res.is_err() { return Err(res.err().unwrap()) }
                    }
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
                    if a == b { self.stack.push(1) } else { self.stack.push(0) }
                }
                TYPES::NE => {
                    if self.stack.len() < 2 { continue }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if a != b { self.stack.push(1) } else { self.stack.push(0) }
                }
                TYPES::LT => {
                    if self.stack.len() < 2 { continue }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if a < b { self.stack.push(1) } else { self.stack.push(0) }
                }
                TYPES::GT => {
                    if self.stack.len() < 2 { continue }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if a > b { self.stack.push(1) } else { self.stack.push(0) }
                }
                TYPES::NOT => {
                    if self.stack.len() < 1 { continue }
                    let a = self.stack.pop().unwrap();
                    if a == 0 { self.stack.push(1) } else { self.stack.push(0) }
                }
                TYPES::PRINT => {
                    if self.stack.len() < 1 { print!(" "); continue }
                    print!("{:?}", self.stack.pop().unwrap());
                }
                TYPES::VAR(var) => {
                    match var.as_str() {
                        "STACK" => println!("{:?}", self.stack.to_vec()),
                        "LENGTH" => self.stack.push(self.stack.len() as isize),
                        _ => {}
                    };
                }
                _ => println!("unknown token {:?}", token.token)
            }
        }
        Ok(())
    }
}

pub fn run(tokens: Vec<Token>) -> Result<Stack<isize>, String> {
    let mut interpreter = Interpreter::new();
    interpreter.std();
    let res = interpreter.interpret(&tokens);
    if res.is_err() { return Err(res.err().unwrap()) }
    Ok(interpreter.stack)
}