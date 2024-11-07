use pest::Parser;
use pest::iterators::Pairs;
use regex_parser::*;

fn main() -> anyhow::Result<()>{
    let input = "a*b";
    let parsed = RegexGrammar::parse(Rule::reg, input)?;
    print_grammar(parsed, 0);
    Ok(())
}

fn print_grammar(pairs: Pairs<Rule>, indent_level: usize) {
    for pair in pairs {
        println!(
            "{}- Rule: {:?}, Text: `{}`",
            "  ".repeat(indent_level),
            pair.as_rule(),
            pair.as_str()
        );
        if pair.clone().into_inner().peek().is_some() {
            print_grammar(pair.into_inner(), indent_level + 1);
        }
    }
}