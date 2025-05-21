use query_compiler::engine::execute_query;
use query_compiler::ast::{Expr, Query, Value, Sort, SortDirection};
use std::fs::File;
use std::io::Write;

/// Creates a basic CSV file for testing.
fn create_test_csv(path: &str) {
    let mut file = File::create(path).unwrap();
    writeln!(file, "name,age,city").unwrap();
    writeln!(file, "Alice,30,Recife").unwrap();
    writeln!(file, "Bob,22,Olinda").unwrap();
    writeln!(file, "Carol,40,Recife").unwrap();
}

/// Builds a minimal query object for tests.
fn build_query() -> Query {
    Query {
        source: "tests/test_data.csv".to_string(),
        filter: Some(Expr::Gt("age".to_string(), Value::Number(25))),
        show: vec!["name".to_string(), "age".to_string()],
        sort: Some(Sort {
            column: "age".to_string(),
            direction: SortDirection::Desc,
        }),
        cap: None,
        map: None,
        unique: false,
    }
}

#[test]
fn executes_query_and_returns_filtered_sorted_dataframe() {
    create_test_csv("tests/test_data.csv");

    let query = build_query();
    let df = execute_query(query).expect("should succeed");

    assert_eq!(df.shape(), (2, 2)); 

    let name_col = df.column("name").unwrap();
    assert_eq!(name_col.str_value(0).unwrap(), "Carol");
    
    let age_col = df.column("age").unwrap();
    assert_eq!(age_col.get(0).unwrap().to_string(), "40");    
}

#[test]
fn returns_empty_if_no_rows_match() {
    create_test_csv("tests/test_data.csv");

    let mut query = build_query();
    query.filter = Some(Expr::Gt("age".to_string(), Value::Number(100)));

    let df = execute_query(query).expect("should succeed");

    assert_eq!(df.height(), 0);
}
