pub mod lexer;
pub mod parser;
pub mod ast;
pub mod engine;
pub mod errors;
pub mod cli;

use lexer::tokenize;
use parser::Parser;
use engine::execute_query;
use errors::QueryError;

pub struct PipelineResult {
    pub tokens: Vec<lexer::Token>,
    pub ast: ast::Query,
    pub output: Option<polars::prelude::DataFrame>,
}

/// Runs the full query pipeline: lexing, parsing, execution.
/// Returns all intermediate results for debugging or further processing.
pub fn run_pipeline(
    input: &str,
    execute: bool,
) -> Result<PipelineResult, QueryError> {
    let tokens = tokenize(input);

    let mut parser = Parser::new(tokens.clone());
    let ast = parser.parse_query()?;

    let output = if execute {
        Some(
            execute_query(ast.clone())
                .map_err(|e| QueryError::Unexpected(e.to_string()))?
        )
    } else {
        None
    };

    Ok(PipelineResult { tokens, ast, output })
}
