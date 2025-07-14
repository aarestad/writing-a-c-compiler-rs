use logos::{Logos, Lexer};

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub(crate) enum NQC {
    #[regex("[a-zA-Z_]\\w*")]
    Text,

    #[regex("[0-9]+")]
    Constant,

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

pub(crate) fn lexer(input: &str) -> Lexer<NQC> {
    NQC::lexer(input)
}
