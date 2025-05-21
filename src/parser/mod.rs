use crate::lexer::Token;
use crate::ast::{Query, Expr, Value, Sort, SortDirection};
use crate::errors::QueryError;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    /// Parses the entire query and returns an AST node.
    pub fn parse_query(&mut self) -> Result<Query, QueryError> {
        self.expect_token(&Token::Source)?;
        let source = match self.next() {
            Some(Token::StringLiteral(s)) => s,
            _ => return Err(QueryError::Expected("Expected string after 'source'".into())),
        };

        let mut filter = None;
        let mut show = Vec::new();
        let mut sort = None;
        let mut cap = None;
        let mut map = None;
        let mut unique = false;

        // The order of keywords is flexible: filter, show, sort, cap, map, unique...
        while let Some(token) = self.peek() {
            match token {
                Token::Filter => {
                    self.next();
                    filter = Some(self.parse_expr()?);
                }
                Token::Show => {
                    self.next();
                    show = self.parse_show_fields()?;
                }
                Token::Sort => {
                    self.next();
                    self.expect_token(&Token::By)?;
                    sort = Some(self.parse_sort()?);
                }
                Token::Cap => {
                    self.next();
                    cap = Some(self.parse_cap()?);
                }
                Token::Map => {
                    self.next();
                    map = Some(self.parse_map()?);
                }
                Token::Unique => {
                    self.next();
                    unique = true;
                }
                Token::Pipe => {
                    self.next();
                    continue; 
                }
                _ => break,
            }
        }

        Ok(Query {
            source,
            filter,
            show,
            sort,
            cap,
            map,
            unique,
        })
    }

    fn parse_show_fields(&mut self) -> Result<Vec<String>, QueryError> {
        let mut fields = Vec::new();
        loop {
            match self.next() {
                Some(Token::Ident(name)) => fields.push(name),
                Some(Token::Comma) => continue,
                Some(_) => {
                    self.position -= 1;
                    break;
                }
                None => break,
            }
        }
        Ok(fields)
    }

    fn parse_sort(&mut self) -> Result<Sort, QueryError> {
        let column = match self.next() {
            Some(Token::Ident(name)) => name,
            _ => return Err(QueryError::Expected("Expected field name after 'sort by'".into())),
        };
        let direction = match self.peek() {
            Some(Token::Ident(dir)) if dir.eq_ignore_ascii_case("desc") => {
                self.next();
                SortDirection::Desc
            }
            Some(Token::Ident(dir)) if dir.eq_ignore_ascii_case("asc") => {
                self.next();
                SortDirection::Asc
            }
            _ => SortDirection::Asc,
        };
        Ok(Sort { column, direction })
    }

    fn parse_cap(&mut self) -> Result<usize, QueryError> {
        match self.next() {
            Some(Token::Number(n)) if n >= 0 => Ok(n as usize),
            _ => Err(QueryError::Expected("Expected positive integer after 'cap'".into())),
        }
    }

    fn parse_map(&mut self) -> Result<(String, Expr), QueryError> {
        // Simple map: map field = expr
        let field = match self.next() {
            Some(Token::Ident(name)) => name,
            _ => return Err(QueryError::Expected("Expected field name after 'map'".into())),
        };
        self.expect_token(&Token::Assign)?;
        let expr = self.parse_comparison_expr()?;
        Ok((field, expr))
    }

    // ----------- NOVO PARSER DE EXPRESSÃO COM PARÊNTESES -----------
    pub fn parse_expr(&mut self) -> Result<Expr, QueryError> {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> Result<Expr, QueryError> {
        let mut left = self.parse_and_expr()?;
        while self.match_ident("or") {
            let right = self.parse_and_expr()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_and_expr(&mut self) -> Result<Expr, QueryError> {
        let mut left = self.parse_comparison_expr()?;
        while self.match_ident("and") {
            let right = self.parse_comparison_expr()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_comparison_expr(&mut self) -> Result<Expr, QueryError> {
        if self.match_token(&Token::LParen) {
            let expr = self.parse_expr()?;
            self.expect_token(&Token::RParen)?;
            return Ok(expr);
        }

        let field = match self.next() {
            Some(Token::Ident(name)) => name,
            _ => return Err(QueryError::Expected("Expected field name in expression".into())),
        };

        let op = match self.next() {
            Some(Token::Eq) => "==",
            Some(Token::NotEq) => "!=",
            Some(Token::Gt) => ">",
            Some(Token::Lt) => "<",
            Some(Token::Gte) => ">=",
            Some(Token::Lte) => "<=",
            _ => return Err(QueryError::Expected("Expected comparison operator".into())),
        };

        let value = match self.next() {
            Some(Token::Float(f)) => Value::Float(f),
            Some(Token::Number(n)) => Value::Number(n),
            Some(Token::StringLiteral(s)) => Value::String(s),
            Some(Token::True) => Value::Bool(true),
            Some(Token::False) => Value::Bool(false),
            Some(tok) => return Err(QueryError::Unexpected(format!("Unexpected value: {:?}", tok))),
            None => return Err(QueryError::Expected("Expected literal value".into())),
        };

        Ok(match op {
            "==" => Expr::Eq(field, value),
            "!=" => Expr::NotEq(field, value),
            ">"  => Expr::Gt(field, value),
            "<"  => Expr::Lt(field, value),
            ">=" => Expr::Gte(field, value),
            "<=" => Expr::Lte(field, value),
            _    => return Err(QueryError::Unexpected("Unknown operator".into())),
        })
    }

    // ------------ HELPERS ------------

    /// Checks for an identifier (e.g., 'and', 'or') and consumes it if found.
    fn match_ident(&mut self, expected: &str) -> bool {
        if let Some(Token::Ident(name)) = self.peek() {
            if name.eq_ignore_ascii_case(expected) {
                self.next();
                return true;
            }
        }
        false
    }

    /// Checks for a token and consumes it if found.
    fn match_token(&mut self, expected: &Token) -> bool {
        if let Some(tok) = self.peek() {
            if tok == expected {
                self.next();
                return true;
            }
        }
        false
    }

    /// Expects a specific token, returns error if not found.
    fn expect_token(&mut self, expected: &Token) -> Result<(), QueryError> {
        match self.next() {
            Some(t) if t == *expected => Ok(()),
            Some(t) => Err(QueryError::Unexpected(format!(
                "Expected token {:?}, found {:?}", expected, t
            ))),
            None => Err(QueryError::Unexpected("Unexpected end of input".into())),
        }
    }

    /// Peeks at the current token without consuming it.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Consumes and returns the current token.
    fn next(&mut self) -> Option<Token> {
        if self.position >= self.tokens.len() {
            None
        } else {
            let tok = self.tokens[self.position].clone();
            self.position += 1;
            Some(tok)
        }
    }
}
