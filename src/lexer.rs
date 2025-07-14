use logos::{Lexer, Logos};

fn valid_constant(lex: &mut Lexer<Nqc>) -> Option<u64> {
    let n: u64 = lex.slice().parse().ok()?;

    if !lex.remainder().chars().next()?.is_whitespace() {
        return None;
    }

    Some(n)
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub(crate) enum Nqc {
    #[regex("[a-zA-Z_]\\w*")]
    Text,

    #[regex("[0-9]+", valid_constant)]
    Constant(u64),

    #[token("int")]
    Int,

    #[token("void")]
    Void,

    #[token("return")]
    Return,

    #[token("(")]
    OpenParen,

    #[token(")")]
    ClosedParen,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    ClosedBrace,

    #[token(";")]
    Semicolon,
}
