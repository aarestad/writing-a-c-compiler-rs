use logos::{Lexer, Logos};

pub(crate) type LexError = String;

fn valid_constant(lex: &mut Lexer<NqcToken>) -> Result<u64, LexError> {
    match lex.slice().parse() {
        Ok(n) => {
            if let Some(lookahead) = lex.remainder().chars().next() {
                if !(lookahead.is_whitespace() || lookahead == ';') {
                    return Err("next char is not whitespace or semicolon".into());
                }

                Ok(n)
            } else {
                Err("unexpected eof".into())
            }
        },

        Err(_) => Err("invalid constant number".into()),
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexError)]
#[logos(skip r"[ \t\n\f]+")]
pub(crate) enum NqcToken {
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
    use super::NqcToken;
    use logos::Logos;

    #[test]
    fn parse_return_two() {
        let src = "int main(void) {\n  return 2;\n}";
        assert_eq!(NqcToken::lexer(src).collect::<Vec<_>>(), vec![
            Ok(NqcToken::Int),
            Ok(NqcToken::Text("main".to_string())),
            Ok(NqcToken::OpenParen),
            Ok(NqcToken::Void),
            Ok(NqcToken::ClosedParen),
            Ok(NqcToken::OpenBrace),
            Ok(NqcToken::Return),
            Ok(NqcToken::Constant(2)),
            Ok(NqcToken::Semicolon),
            Ok(NqcToken::ClosedBrace),
        ]);
    }

    #[test]
    fn fails_on_2abc() {
        let src = "int main(void) {\n  return 2abc;\n}";
        assert_eq!(NqcToken::lexer(src).collect::<Vec<_>>(), vec![
            Ok(NqcToken::Int),
            Ok(NqcToken::Text("main".to_string())),
            Ok(NqcToken::OpenParen),
            Ok(NqcToken::Void),
            Ok(NqcToken::ClosedParen),
            Ok(NqcToken::OpenBrace),
            Ok(NqcToken::Return),
            Err("next char is not whitespace or semicolon".into()),
            Ok(NqcToken::Text("abc".to_string())),
            Ok(NqcToken::Semicolon),
            Ok(NqcToken::ClosedBrace),
        ]);
    }
}
