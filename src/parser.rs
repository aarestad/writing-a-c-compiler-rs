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

fn token_from_opt_result(tr_opt: Option<&TokenResult>) -> Result<&NqcToken, CompilerError> {
    match tr_opt {
        Some(tr) => {
            match tr {
                Ok(token) => {
                    Ok(token)
                },
                Err(lex_err) => Err(lex_err.clone())
            }
        },
        None => Err(CompilerError::ParserError("out of tokens!".into()))
    }
}

fn parse_function(tokens: Vec<TokenResult>) -> Result<Box<NqcNode>, CompilerError> {
    let mut tok_iter = tokens.iter();

    let int_tok = token_from_opt_result(tok_iter.next())?;
    if int_tok != &NqcToken::Int {
        return Err(CompilerError::ParserError(format!("expected Int, got {:?}", int_tok).into()));
    }

    let text_tok = token_from_opt_result(tok_iter.next())?;
    let NqcToken::Text(identifier) = text_tok else {
        return Err(CompilerError::ParserError(format!("expected Text, got {:?}", text_tok).into()));
    };

    let lp_tok = token_from_opt_result(tok_iter.next())?;
    if lp_tok != &NqcToken::OpenParen {
        return Err(CompilerError::ParserError(format!("expected lparen, got {:?}", lp_tok).into()));
    }

    let void_tok = token_from_opt_result(tok_iter.next())?;
    if void_tok != &NqcToken::Void {
        return Err(CompilerError::ParserError(format!("expected void, got {:?}", void_tok).into()));
    }

    let rp_tok = token_from_opt_result(tok_iter.next())?;
    if rp_tok != &NqcToken::ClosedParen {
        return Err(CompilerError::ParserError(format!("expected rparen, got {:?}", rp_tok).into()));
    }

    let lb_tok = token_from_opt_result(tok_iter.next())?;
    if lb_tok != &NqcToken::OpenBrace {
        return Err(CompilerError::ParserError(format!("expected lbrace, got {:?}", lb_tok).into()));
    }

    let rest_of_toks = tok_iter.cloned().collect::<Vec<_>>();

    let rb_tok = token_from_opt_result(rest_of_toks.last())?;
    if rb_tok != &NqcToken::ClosedBrace {
        return Err(CompilerError::ParserError(format!("expected tbrace, got {:?}", lb_tok).into()));
    }

    Ok(Box::new(NqcNode::Function(identifier.into(), parse_statement(rest_of_toks[0..rest_of_toks.len()-1].to_vec())?)))
}

fn parse_statement(tokens: Vec<TokenResult>) -> Result<Box<NqcNode>, CompilerError> {
    let mut tok_iter = tokens.iter();

    let return_tok = token_from_opt_result(tok_iter.next())?;
    if return_tok != &NqcToken::Return {
        return Err(CompilerError::ParserError(format!("expected return, got {:?}", return_tok).into()));
    }

    todo!()
}

pub(crate) fn parse(tokens: Vec<TokenResult>) -> Result<NqcNode, CompilerError> {
    Ok(NqcNode::Program(parse_function(tokens)?))
}
