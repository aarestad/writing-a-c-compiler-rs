use std::{env, fs};

mod lexer;

fn main() {
    let filename = env::args().nth(1).expect("Expected file argument");
    let src = fs::read_to_string(&filename)
        .expect(&format!("Failed to read file {}", &filename));

    let mut lexer = lexer::lexer(&src);

    loop {
        if let Some(tok) = lexer.next() {
            match tok {
                Err(_) => panic!("bad token: {:?}", lexer.slice()),
                Ok(t) => println!("{:#?}", t),
            }
        } else {
            break;
        }
    }
}
