use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct Stack<T> {
    stack: Vec<T>,
} impl<T> Stack<T> {
    pub fn new() -> Self { Stack { stack: Vec::new() } }
    pub fn len(&self) -> usize { self.stack.len() }
    pub fn pop(&mut self) -> Option<T> { self.stack.pop() }
    pub fn push(&mut self, item: T) { self.stack.push(item) }
    pub fn is_empty(&self) -> bool { self.stack.is_empty() }
    pub fn peek(&self) -> Option<&T> { self.stack.last() }
    pub fn to_vec(&self) -> &Vec<T> { &self.stack }
}