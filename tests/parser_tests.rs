use query_compiler::parser::Parser;
use query_compiler::ast::{Expr, OrderDirection, Query};
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
        from "data.csv"
        select name, age
        where age > 20
        order by age desc
    "#);

    assert_eq!(query.from, "data.csv");
    assert_eq!(query.select, vec!["name", "age"]);
    assert!(matches!(query.where_clause.as_ref().unwrap(), Expr::Gt(_, _)));
    assert_eq!(query.order_by.as_ref().unwrap().column, "age");
    assert!(matches!(query.order_by.unwrap().direction, OrderDirection::Desc));
}

#[test]
fn parses_and_or_expression() {
    let query = parse_ok(r#"
        from "data.csv"
        select a
        where a == 1 and b != 2 or c >= 3
    "#);

    match query.where_clause.unwrap() {
        Expr::Or(left, right) => {
            assert!(matches!(*left, Expr::And(_, _)));
            assert!(matches!(*right, Expr::Gte(_, _)));
        }
        _ => panic!("Expected OR expression"),
    }
}

#[test]
fn parses_order_by_with_implicit_asc() {
    let query = parse_ok(r#"
        from "data.csv"
        select name
        order by name
    "#);

    assert_eq!(query.order_by.unwrap().direction, OrderDirection::Asc);
}

#[test]
fn parses_multiple_select_columns() {
    let query = parse_ok(r#"
        from "data.csv"
        select col1, col2, col3
    "#);

    assert_eq!(query.select, vec!["col1", "col2", "col3"]);
}

#[test]
fn fails_when_missing_from_clause() {
    let tokens = tokenize("select x");
    let mut parser = Parser::new(tokens);
    assert!(parser.parse_query().is_err());
}
