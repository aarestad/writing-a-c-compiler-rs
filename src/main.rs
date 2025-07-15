use std::{env, fs};
use crate::lexer::NqcToken;
use logos::Logos;
use crate::parser::parse;

mod lexer;
mod parser;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).unwrap_or("samples/return_2.c".into());
    let src = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read file {}", &filename));

    let tokens = NqcToken::lexer(&src).collect::<Vec<_>>();
    println!("{tokens:?}");

    let ast = parse(tokens)?;
    println!("{ast:?}");

    Ok(())
}
