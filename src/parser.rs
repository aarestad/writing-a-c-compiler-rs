use crate::errors::CompilerError;
use crate::lexer::{NqcToken, TokenResult};

#[derive(Debug, PartialEq)]
pub(crate) enum NqcNode {
    Program(Box<NqcNode>),
    Function(String, Box<NqcNode>),
    Return(Box<NqcNode>),
    Constant(u64),
}

fn parse_recurse(tokens: Vec<TokenResult>) -> Result<Box<NqcNode>, CompilerError> {
    if let Some(token_result) = tokens.first() {
        if let Ok(token) = token_result {
            match token {
                NqcToken::Text(text) => Ok(Box::new(NqcNode::Function(text.into(), parse_recurse(tokens[1..].into())?))),
                NqcToken::Constant(value) => Ok(Box::new(NqcNode::Constant(*value))),
                NqcToken::Return => Ok(Box::new(NqcNode::Return(parse_recurse(tokens[1..].into())?))),
                _ => parse_recurse(tokens[1..].into()), // ignoring the rest for now
            }
        } else {
            Err(token_result.clone().unwrap_err())
        }
    } else {
        Err(CompilerError::ParserError("out of tokens!".into()))
    }
}

pub(crate) fn parse(tokens: Vec<TokenResult>) -> Result<NqcNode, CompilerError> {
    Ok(NqcNode::Program(parse_recurse(tokens)?))
}
