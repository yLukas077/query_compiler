mod lexer;
mod parser;
mod ast;
mod engine;
mod errors;

use clap::Parser as ClapParser;
use std::fs;
use std::path::PathBuf;
use crate::lexer::tokenize;
use crate::parser::Parser;
use crate::engine::execute_query;

/// CLI arguments for the CSV Query Next-Gen.
#[derive(ClapParser, Debug)]
#[command(name = "csv-query", about = "Compile and execute next-gen queries on CSV data.")]
pub struct CliArgs {
    #[arg(short, long)]
    pub query: PathBuf,

    #[arg(short, long)]
    pub output: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    pub tokens: bool,

    #[arg(long, default_value_t = false)]
    pub ast: bool,

    #[arg(long, default_value_t = false)]
    pub show: bool,
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let query_str = fs::read_to_string(&args.query)?;

    let tokens = tokenize(&query_str);
    if args.tokens {
        println!("# Tokens:");
        for t in &tokens {
            println!("{:?}", t);
        }
    }

    let mut parser = Parser::new(tokens.clone());
    let ast = parser.parse_query()?;
    if args.ast {
        println!("\n# AST:");
        println!("{:#?}", ast);
    }

    let result_df = execute_query(ast.clone())?;
    if args.show {
        println!("\n# Result DataFrame:");
        println!("{}", &result_df);
    }

    if let Some(out_path) = &args.output {
        let ext = out_path.extension().and_then(|x| x.to_str()).unwrap_or("csv").to_lowercase();
        let file_name = out_path.file_name().and_then(|x| x.to_str()).unwrap_or("result");
        let (dir, full_path) = match ext.as_str() {
            "csv" => ("reports/csv", format!("reports/csv/{}", file_name)),
            "json" => ("reports/json", format!("reports/json/{}", file_name)),
            _ => ("reports", format!("reports/{}", file_name)),
        };
        std::fs::create_dir_all(dir)?;
        let mut file = std::fs::File::create(&full_path)?;
    
        match ext.as_str() {
            "csv" => {
                use polars::prelude::*;
                let mut writer = CsvWriter::new(&mut file);
                writer.finish(&mut result_df.clone())?;
                println!("Exported result to {}", full_path);
            }
            "json" => {
                use std::io::Write;
                use serde_json::json;
                let columns = result_df.get_columns();
                let headers: Vec<&str> = result_df.get_column_names();
                let mut records = vec![];
    
                for i in 0..result_df.height() {
                    let mut record = serde_json::Map::new();
                    for (col, name) in columns.iter().zip(headers.iter()) {
                        let value = col.get(i);
                        let json_value = match value {
                            Ok(polars::prelude::AnyValue::String(s)) => json!(s),
                            Ok(polars::prelude::AnyValue::Int64(n)) => json!(n),
                            Ok(polars::prelude::AnyValue::UInt64(n)) => json!(n),
                            Ok(polars::prelude::AnyValue::Float64(f)) => json!(f),
                            Ok(polars::prelude::AnyValue::Float32(f)) => json!(f),
                            Ok(polars::prelude::AnyValue::Int32(n)) => json!(n),
                            Ok(polars::prelude::AnyValue::Boolean(b)) => json!(b),
                            Ok(ref v) => json!(v.to_string()),
                            Err(_) => json!(null),
                        };
                        record.insert((*name).to_string(), json_value);
                    }
                    records.push(serde_json::Value::Object(record));
                }
    
                let json = serde_json::to_string_pretty(&records)?;
                file.write_all(json.as_bytes())?;
                println!("Exported result to {}", full_path);
            }
            _ => {
                eprintln!("Unsupported output format: {}", ext);
            }
        }
    }
    

    Ok(())
}
