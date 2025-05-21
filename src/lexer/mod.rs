use logos::Logos;

/// Tokens for the next-gen CSV query language.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    #[token("source")]
    Source,
    #[token("show")]
    Show,
    #[token("filter")]
    Filter,
    #[token("sort")]
    Sort,
    #[token("by")]
    By,
    #[token("cap")]
    Cap,
    #[token("map")]
    Map,
    #[token("unique")]
    Unique,
    #[token("merge")]
    Merge,
    #[token("alias")]
    Alias,
    #[token("cluster")]
    Cluster,
    #[token("with")]
    With,
    #[token("exclude")]
    Exclude,
    #[token("fuzzy")]
    Fuzzy,
    #[token("watch")]
    Watch,
    #[token("on_error")]
    OnError,
    #[token("true")]
    True,
    #[token("false")]
    False,


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
    #[token("|>")]
    Pipe,

    // Literals
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice().trim_matches('"').to_string())]
    StringLiteral(String),

    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse().unwrap_or(0.0))]
    Float(f64),

    #[regex(r"[0-9]+", |lex| lex.slice().parse().unwrap_or(0))]
    Number(i64),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    // Ignored: whitespace and comments
    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    #[regex(r"#.*", logos::skip)]
    Whitespace,
}

/// Tokenizes the input query text into a list of tokens.
pub fn tokenize(input: &str) -> Vec<Token> {
    Token::lexer(input)
        .filter_map(Result::ok)
        .collect()
}
