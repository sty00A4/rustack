#![allow(dead_code)]
#![allow(unused)]
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};
use std::process::exit;

mod lexer;
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

fn main() {
    let file_name = String::from("test/test.rst");
    let text = read_file(file_name.as_str());
    let lex_res = lexer::lex(file_name, text);
    if lex_res.is_err() { eprintln!("{}", lex_res.err().unwrap()); return }
    let tokens = lex_res.unwrap();
    for token in tokens { println!("{:?}", token); }
}