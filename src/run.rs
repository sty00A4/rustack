use crate::lexer::{Token, TYPES};
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