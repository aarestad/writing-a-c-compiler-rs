use crate::errors::CompilerError;
use crate::lexer::{NqcToken, TokenResult};

pub(crate) type ParseError = String;

#[derive(Debug, PartialEq)]
pub(crate) enum NqcNode {
    Program(Box<NqcNode>),
    Function(String, Box<NqcNode>),
    Return(Box<NqcNode>),
    Constant(u64),
}

pub(crate) fn parse(tokens: Vec<TokenResult>) -> Result<NqcNode, CompilerError> {
    let Some(fn_result) = tokens.iter().find(|t| {
        if let Ok(NqcToken::Text(_)) = t {
            return true;
        }

        false
    }) else {
        return Err(CompilerError::ParserError(
            "could not find function name".into(),
        ));
    };

    let Ok(NqcToken::Text(fn_name)) = fn_result else {
        return Err(CompilerError::ParserError(format!(
            "text parse error: {:#?}",
            &fn_result.clone().err().unwrap()
        )));
    };

    let Some(value) = tokens.iter().find(|t| {
        if let Ok(NqcToken::Constant(_)) = t {
            return true;
        }

        false
    }) else {
        return Err(CompilerError::ParserError("could not find constant".into()));
    };

    let Ok(NqcToken::Constant(value)) = value else {
        return Err(CompilerError::ParserError(format!(
            "constant parse error: {:#?}",
            fn_result.clone().err().unwrap()
        )));
    };

    let constant_node = NqcNode::Constant(*value);
    let return_node = NqcNode::Return(Box::new(constant_node));
    let fn_node = NqcNode::Function(fn_name.into(), Box::new(return_node));

    Ok(NqcNode::Program(Box::new(fn_node)))
}
