use crate::lexer::{Token, TYPES};
use crate::read_file;
use crate::stack::Stack;

struct Interpreter {
    stack: Stack<isize>
} impl Interpreter {
    pub fn new() -> Self { Self { stack: Stack::new() } }
    pub fn interpret(&mut self, tokens: &Vec<Token>) -> Result<(), String> {
        for token in tokens {
            match &token.token {
                TYPES::INT(value) => self.stack.push(*value),
                TYPES::BODY(tokens_) => self.interpret(tokens_).unwrap(),
                TYPES::REPEAT(tokens_) => {
                    if self.stack.len() < 1 { continue }
                    let a = self.stack.pop().unwrap();
                    for i in 0..a {
                        self.interpret(tokens_).unwrap();
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
    interpreter.interpret(&tokens).unwrap();
    Ok(interpreter.stack)
}