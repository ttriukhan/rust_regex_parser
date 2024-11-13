use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Parse a regex pattern from a file
    Parse {
        /// The file containing the regex pattern to parse
        #[structopt()]
        file: String,
    },
    /// Display credits information
    Credits,
    /// Display custom help information
    Help,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Regex Parser",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}
