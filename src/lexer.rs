use logos::{Lexer, Logos};

fn valid_constant(lex: &mut Lexer<Nqc>) -> Option<u64> {
    let n: u64 = lex.slice().parse().ok()?;

    let lookahead = lex.remainder().chars().next()?;

    if !(lookahead.is_whitespace() || lookahead == ';') {
        return None;
    }

    Some(n)
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub(crate) enum Nqc {
    #[regex("[a-zA-Z_]\\w*", |lex| lex.slice().parse().ok())]
    Text(String),

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

#[cfg(test)]
mod test {
    use super::Nqc;
    use logos::Logos;

    #[test]
    fn parse_return_two() {
        let src = "int main(void) {\n  return 2;\n}";
        assert_eq!(Nqc::lexer(src).collect::<Vec<_>>(), vec![
            Ok(Nqc::Int),
            Ok(Nqc::Text("main".to_string())),
            Ok(Nqc::OpenParen),
            Ok(Nqc::Void),
            Ok(Nqc::ClosedParen),
            Ok(Nqc::OpenBrace),
            Ok(Nqc::Return),
            Ok(Nqc::Constant(2)),
            Ok(Nqc::Semicolon),
            Ok(Nqc::ClosedBrace),
        ]);
    }

    #[test]
    fn fails_on_2abc() {
        let src = "int main(void) {\n  return 2abc;\n}";
        assert_eq!(Nqc::lexer(src).collect::<Vec<_>>(), vec![
            Ok(Nqc::Int),
            Ok(Nqc::Text("main".to_string())),
            Ok(Nqc::OpenParen),
            Ok(Nqc::Void),
            Ok(Nqc::ClosedParen),
            Ok(Nqc::OpenBrace),
            Ok(Nqc::Return),
            Err(()),
            Ok(Nqc::Text("abc".to_string())),
            Ok(Nqc::Semicolon),
            Ok(Nqc::ClosedBrace),
        ]);
    }
}
