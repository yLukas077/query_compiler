use clap::Parser;
use std::path::PathBuf;

/// CLI arguments for the Query Compiler.
#[derive(Parser, Debug)]
#[command(name = "csv-query", about = "Compile and execute .query files on CSV data")]
pub struct CliArgs {
    pub query_path: PathBuf,
    pub output: Option<PathBuf>,
}

impl CliArgs {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
