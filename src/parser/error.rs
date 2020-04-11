use crate::lexer::Token;
use crate::util::Span;
use thiserror::Error;

// TODO: Add more error type
/// 
/// Represent possible error that can happen while parsing, it should be very similar to normal JSON's syntax error message as possible
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ParseError<'a> {
	/// This error is raise when the parser expect another [Token](../lexer/struct.Token.html)
	/// but reach the end of file instead
	#[error("Unexpected End-Of-File at {0}")]
	UnexpectedEndOfFile(Span),

	/// This error is raise when parser is trying to parse a [Token](../lexer/struct.Token.html) that isn't in JSON syntax specification
	#[error("Invalid Token at {0}")]
	InvalidToken(Token<'a>),

	/// This error is raise when parser is expecting a certain [Token](../lexer/struct.Token.html) but found something else.
	#[error("Unexpected Token: {0}, expecting: {1}")]
	UnexpectedToken(Token<'a>, Token<'a>),
}
