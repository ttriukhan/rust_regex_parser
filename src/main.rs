mod cli;
use cli::{Action::*, CommandLineArgs};
use pest::iterators::Pairs;
use pest::Parser;
use regex_parser::*;
use std::fs;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action } = CommandLineArgs::from_args();
    match action {
        Parse { file } => parse_file(&file)?,
        Credits => display_credits(),
        Help => display_help(),
    }
    Ok(())
}

fn parse_file(file_name: &str) -> anyhow::Result<()> {
    let input = fs::read_to_string(file_name)?;
    let parsed = RegexGrammar::parse(Rule::reg, &input).map_err(RegexParsingError::from)?;
    print_result(parsed);
    Ok(())
}

fn display_credits() {
    println!("Regex CLI Parser by Tania Triukhan (ttriukhan)");
    println!("Powered by pest for parsing and clap for CLI management.");
}

fn display_help() {
    println!("Regex CLI Parser - Custom Help");
    println!("Commands:");
    println!("  parse <fileName>    Parses the regex pattern contained in the specified file");
    println!("  help                Displays this help information");
    println!("  credits             Displays credits information");
}

fn print_result(pairs: Pairs<Rule>) {
    if let Some(first_pair) = pairs.clone().next() {
        println!("{}", first_pair.as_str());
    }
}