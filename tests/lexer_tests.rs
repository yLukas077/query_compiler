use query_compiler::lexer::{tokenize, Token};

#[test]
fn parses_keywords_and_identifiers() {
    let input = r#"source "data.csv" show name, age filter age > 18"#;
    let tokens = tokenize(input);

    assert_eq!(tokens[0], Token::Source);
    assert_eq!(tokens[1], Token::StringLiteral("data.csv".into()));
    assert_eq!(tokens[2], Token::Show);
    assert_eq!(tokens[3], Token::Ident("name".into()));
    assert_eq!(tokens[4], Token::Comma);
    assert_eq!(tokens[5], Token::Ident("age".into()));
    assert_eq!(tokens[6], Token::Filter);
    assert_eq!(tokens[7], Token::Ident("age".into()));
    assert_eq!(tokens[8], Token::Gt);
    assert_eq!(tokens[9], Token::Number(18));
}

#[test]
fn skips_whitespace_and_comments() {
    let input = r#"
        # comment
        source   "data.csv"    show name # another comment
    "#;

    let tokens = tokenize(input);
    assert_eq!(tokens[0], Token::Source);
    assert_eq!(tokens[1], Token::StringLiteral("data.csv".into()));
    assert_eq!(tokens[2], Token::Show);
    assert_eq!(tokens[3], Token::Ident("name".into()));
}

#[test]
fn parses_operators_correctly() {
    let input = r#"x == 1 y != 2 z >= 3 w <= 4 a > 5 b < 6"#;
    let tokens = tokenize(input);

    use Token::*;
    let expected = vec![
        Ident("x".into()), Eq, Number(1),
        Ident("y".into()), NotEq, Number(2),
        Ident("z".into()), Gte, Number(3),
        Ident("w".into()), Lte, Number(4),
        Ident("a".into()), Gt, Number(5),
        Ident("b".into()), Lt, Number(6),
    ];

    assert_eq!(tokens, expected);
}
