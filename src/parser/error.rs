use thiserror::Error;
use crate::util::Span;
use crate::lexer::Token;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ParseError<'a> {
	#[error("Unexpected End-Of-File at {0}")]
	UnexpectedEndOfFile(Span),

	#[error("Invalid Token at {0}")]
	InvalidToken(Token<'a>),

	#[error("Unexpected Token: {0}, expecting: {1}")]
	UnexpectedToken(Token<'a>, Token<'a>)
}