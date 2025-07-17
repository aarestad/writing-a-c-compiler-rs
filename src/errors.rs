#[derive(Debug, PartialEq, Clone, Default)]
pub(crate) enum CompilerError {
    #[default]
    Unknown,
    LexerError(String),
    ParserError(String),
}
