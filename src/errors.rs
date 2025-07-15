use crate::lexer::LexError;
use crate::parser::ParseError;

#[derive(Debug, PartialEq, Clone, Default)]
pub(crate) enum CompilerError {
    #[default]
    Unknown,
    LexerError(LexError),
    ParserError(ParseError),
}
