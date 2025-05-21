use logos::Logos;

/// Tokens used in the CSV query language.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    #[token("from")]
    From,
    #[token("where")]
    Where,
    #[token("select")]
    Select,
    #[token("order")]
    Order,
    #[token("by")]
    By,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("not")]
    Not,
    #[token("to")]
    To,

    // Symbols and operators
    #[token("==")]
    Eq,
    #[token("!=")]
    NotEq,
    #[token(">")]
    Gt,
    #[token("<")]
    Lt,
    #[token("=")]
    Assign,
    #[token(">=")]
    Gte,
    #[token("<=")]
    Lte,
    #[token(",")]
    Comma,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    // Literals
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice().trim_matches('"').to_string())]
    StringLiteral(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse().unwrap_or(0))]
    Number(i64),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    // Ignored: whitespace and comments
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r"#.*", logos::skip)]
    Whitespace,

    #[allow(dead_code)]
    Error,
}

/// Tokenizes the input `.query` text into a list of tokens.
pub fn tokenize(input: &str) -> Vec<Token> {
    Token::lexer(input)
        .filter_map(Result::ok)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = r#"
            from "clientes.csv"
            where idade > 25 and cidade == "Recife"
            select nome, email
        "#;

        let tokens = tokenize(input);

        for token in &tokens {
            println!("{:?}", token);
        }

        assert!(tokens.contains(&Token::From));
        assert!(tokens.contains(&Token::Where));
        assert!(tokens.contains(&Token::Select));
        assert!(tokens.contains(&Token::StringLiteral("clientes.csv".to_string())));
        assert!(tokens.contains(&Token::Ident("idade".to_string())));
    }
}
