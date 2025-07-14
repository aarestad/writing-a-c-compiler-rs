use std::{env, fs};
use crate::lexer::Nqc;
use logos::Logos;

mod lexer;

fn main() {
    let filename = env::args().nth(1).expect("Expected file argument");
    let src = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read file {}", &filename));

    println!("{:?}", Nqc::lexer(&src).collect::<Vec<_>>());
}
