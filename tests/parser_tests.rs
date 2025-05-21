use query_compiler::parser::Parser;
use query_compiler::ast::{Expr, SortDirection, Query};
use query_compiler::lexer::tokenize;

/// Helper to parse valid queries and panic if parsing fails.
fn parse_ok(input: &str) -> Query {
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    parser.parse_query().expect("should parse")
}

#[test]
fn parses_basic_query() {
    let query = parse_ok(r#"
        source "data.csv"
        show name, age
        filter age > 20
        sort by age desc
    "#);

    assert_eq!(query.source, "data.csv");
    assert_eq!(query.show, vec!["name", "age"]);
    assert!(matches!(query.filter.as_ref().unwrap(), Expr::Gt(_, _)));
    assert_eq!(query.sort.as_ref().unwrap().column, "age");
    assert!(matches!(query.sort.unwrap().direction, SortDirection::Desc));
}

#[test]
fn parses_and_or_expression() {
    let query = parse_ok(r#"
        source "data.csv"
        show a
        filter a == 1 and b != 2 or c >= 3
    "#);

    match query.filter.unwrap() {
        Expr::Or(left, right) => {
            assert!(matches!(*left, Expr::And(_, _)));
            assert!(matches!(*right, Expr::Gte(_, _)));
        }
        _ => panic!("Expected OR expression"),
    }
}

#[test]
fn parses_sort_by_with_implicit_asc() {
    let query = parse_ok(r#"
        source "data.csv"
        show name
        sort by name
    "#);

    assert_eq!(query.sort.unwrap().direction, SortDirection::Asc);
}

#[test]
fn parses_multiple_show_columns() {
    let query = parse_ok(r#"
        source "data.csv"
        show col1, col2, col3
    "#);

    assert_eq!(query.show, vec!["col1", "col2", "col3"]);
}

#[test]
fn fails_when_missing_source_clause() {
    let tokens = tokenize("show x");
    let mut parser = Parser::new(tokens);
    assert!(parser.parse_query().is_err());
}
