use pest::error::Error as PestError;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
///A simple grammar for parsing standard regular expressions
pub struct RegexGrammar;

#[derive(Debug, Error)]
#[error("Parsing error occurred: {0}")]
/// An error that can occur during parsing standard regular expressions.
pub struct RegexParsingError(#[from] PestError<Rule>);
