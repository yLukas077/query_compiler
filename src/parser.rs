use crate::lexer::Token;
use crate::ast::{Query, Expr, Value, Order, OrderDirection};
use crate::errors::QueryError;

/// Parser for `.query` files.
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Creates a new parser from a list of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    /// Parses the full query and returns a `Query` AST node.
    pub fn parse_query(&mut self) -> Result<Query, QueryError> {
        self.expect_token(&Token::From)?;
        let from = match self.next() {
            Some(Token::StringLiteral(s)) => s.trim_matches('"').to_string(),
            _ => return Err(QueryError::Expected("Expected string after `from`".into())),
        };

        self.expect_token(&Token::Select)?;
        let mut select = Vec::new();
        loop {
            match self.next() {
                Some(Token::Ident(name)) => select.push(name),
                Some(Token::Comma) => continue,
                Some(_unexpected) => {
                    self.position -= 1;
                    break;
                }
                None => break,
            }
        }

        let where_clause = if self.match_token(&Token::Where) {
            Some(self.parse_expr()?)
        } else {
            None
        };

        let order_by = if self.match_token(&Token::Order) {
            self.expect_token(&Token::By)?;
            Some(self.parse_order_by()?)
        } else {
            None
        };

        Ok(Query {
            from,
            select,
            where_clause,
            order_by,
        })
    }

    /// Parses chained logical expressions (`and`, `or`).
    fn parse_expr(&mut self) -> Result<Expr, QueryError> {
        let mut left = self.parse_simple_expr()?;

        while let Some(op) = self.peek() {
            match op {
                Token::And => {
                    self.next();
                    let right = self.parse_simple_expr()?;
                    left = Expr::And(Box::new(left), Box::new(right));
                }
                Token::Or => {
                    self.next();
                    let right = self.parse_simple_expr()?;
                    left = Expr::Or(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parses a simple comparison expression like `field > value`.
    fn parse_simple_expr(&mut self) -> Result<Expr, QueryError> {
        let field = match self.next() {
            Some(Token::Ident(name)) => name,
            _ => return Err(QueryError::Expected("Expected field name".into())),
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
            Some(Token::Number(n)) => Value::Number(n),
            Some(Token::StringLiteral(s)) => Value::String(s),
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

    /// Parses the `order by` clause.
    fn parse_order_by(&mut self) -> Result<Order, QueryError> {
        let column = match self.next() {
            Some(Token::Ident(name)) => name,
            _ => return Err(QueryError::Expected("Expected column name after `order by`".into())),
        };

        let direction = match self.peek() {
            Some(Token::Ident(dir)) if dir.to_lowercase() == "desc" => {
                self.next();
                OrderDirection::Desc
            }
            Some(Token::Ident(dir)) if dir.to_lowercase() == "asc" => {
                self.next();
                OrderDirection::Asc
            }
            _ => OrderDirection::Asc,
        };

        Ok(Order { column, direction })
    }

    /// Expects a specific token; returns an error if not found.
    fn expect_token(&mut self, expected: &Token) -> Result<(), QueryError> {
        match self.next() {
            Some(t) if t == *expected => Ok(()),
            Some(t) => Err(QueryError::Unexpected(format!(
                "Expected token {:?}, but found {:?}", expected, t
            ))),
            None => Err(QueryError::Unexpected("Unexpected end of input".into())),
        }
    }

    /// Tries to match and consume a specific token.
    fn match_token(&mut self, expected: &Token) -> bool {
        if let Some(tok) = self.peek() {
            if tok == expected {
                self.next();
                return true;
            }
        }
        false
    }

    /// Returns the current token without consuming it.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Returns the current token and advances the position.
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
