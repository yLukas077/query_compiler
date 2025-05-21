mod cli;
mod lexer;
mod ast;
mod parser;
mod errors;
mod engine;

use cli::CliArgs;
use std::fs;
use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    let input = fs::read_to_string(&args.query_path)?;
    println!("Query:\n{}\n", input);

    let tokens = lexer::tokenize(&input);
    println!("\nTokens:");
    for token in &tokens {
        println!("{:?}", token);
    }

    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse_query()?;

    let result_df = engine::execute_query(ast)?;

    if let Some(out_path) = args.output {
        let export_path = std::path::Path::new("reports").join(out_path);
        std::fs::create_dir_all("reports")?;

        let mut file = std::fs::File::create(&export_path)?;
        let mut writer = CsvWriter::new(&mut file);
        writer.finish(&mut result_df.clone())?;

        println!("\nResult exported to: {}", export_path.display());
    } else {
        println!("\nResult:");
        println!("{}", result_df);
    }

    Ok(())
}
