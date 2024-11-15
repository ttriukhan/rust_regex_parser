# regex_parser
Link: https://crates.io/crates/regex_parser Docs: https://docs.rs/regex_parser/0.1.1/regex_parser/

This project provides a parser for standard regular expressions (regex) based on a custom grammar. The parser is capable of recognizing regex patterns that use the following components:

- | (alternation): allows for multiple pattern options
- *, +, ? (quantifiers): indicate zero or more, one or more, or zero or one occurrences, respectively
- () (grouping): allows grouping of sub-patterns for precedence or repeated application
- Literal characters: matches any character except special characters (, ), |, *, +, and ?

## parsing process

The parser reads a regex pattern as input and processes it according to the following grammar rules:

1. reg ::= rexpr 'eos': A complete regex expression followed by end-of-string.
2. rexpr ::= rterm { '|' rterm }: An expression consisting of one or more terms, separated by |.
3. rterm ::= rfact { rfact }: A term consisting of one or more factors.
4. rfact ::= prime ['*' | '+' | '?']: A factor, optionally followed by one of the quantifiers *, +, or ?.
5. prime ::= rsymb | '(' rexpr ')': A primary element, which is either a single symbol or a grouped expression.
6. rsymb ::= any character except '(', ')', '|', '*', '+', '?' : Any character except the reserved symbols for regex operations.

## usage of parsing results

Now usage functions are not provided, but in next iterations once parsed, the regex could be validated, interpreted, or transformed into other structures, such as finite automata or abstract syntax trees, for use in matching or searching algorithms.

## unit tests

Unit tests are provided to ensure that the parser correctly interprets various regex patterns.
