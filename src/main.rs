#![allow(dead_code)]
#![allow(unused)]
mod stack;
mod lexer;
mod run;

use stack::Stack;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};
use std::process::exit;
pub fn read_file(path: &str) -> String {
    let f = match File::open(path) {
        Ok(v) => v,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound { eprintln!("couldn't find file path '{}'", path) }
            else { eprintln!("{}", e) }
            exit(1)
        }
    };
    let mut lines = BufReader::new(f).lines();
    let mut text = String::new();
    loop {
        if let Some(s) = lines.next() { text.push_str(s.unwrap().as_str()); }
        else { break }
        text.push_str("\n");
    }
    text.pop();
    return text;
}

struct Flags {
    stack: bool,
    tokens: bool,
} impl Flags {
    pub fn new() -> Self {
        Self {
            stack: false,
            tokens: false,
        }
    }
}

fn main() {
    // FLAGS
    let mut flags = Flags::new();
    let mut args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        args.push(String::from("test/test.rst"));
    }
    if args.len() > 2 {
        let mut input_flags: Vec<String> = Vec::new();
        for i in 2..args.len() { input_flags.push((&args[i]).clone()) }
        for flag in input_flags {
            match flag.as_str() {
                "--stack" => flags.stack = true,
                "--tokens" => flags.tokens = true,
                _ => { eprintln!("unsupported '{}'", &args[2]);return; }
            }
        }
    }
    // FILE
    let file_name = &args[1];
    let text = read_file(file_name.as_str());
    // LEX
    let lex_res = lexer::lex(&file_name, text);
    if lex_res.is_err() { eprintln!("{}", lex_res.err().unwrap()); return }
    let tokens = lex_res.unwrap();
    if flags.tokens { println!(); for token in &tokens { println!("{:?}", token); } println!(); }
    // RUN
    let res = run::run(tokens);
    if res.is_err() { eprintln!("{}", res.err().unwrap()); return }
    let stack = res.unwrap();
    if flags.stack { println!("{:?}", stack.to_vec()) };
    return
}