use pest::error::Error as PestError;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct RegexGrammar;

#[derive(Debug, Error)]
#[error("Parsing error occurred: {0}")]
pub struct RegexParsingError(#[from] PestError<Rule>);
