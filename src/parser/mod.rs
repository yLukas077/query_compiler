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
        let expr = self.parse_simple_expr()?;
        Ok((field, expr))
    }

    fn parse_expr(&mut self) -> Result<Expr, QueryError> {
        // Handles chained logical expressions: age > 20 and city == "Recife"
        let mut left = self.parse_simple_expr()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Ident(op) if op.eq_ignore_ascii_case("and") => {
                    self.next();
                    let right = self.parse_simple_expr()?;
                    left = Expr::And(Box::new(left), Box::new(right));
                }
                Token::Ident(op) if op.eq_ignore_ascii_case("or") => {
                    self.next();
                    let right = self.parse_simple_expr()?;
                    left = Expr::Or(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_simple_expr(&mut self) -> Result<Expr, QueryError> {
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

    fn expect_token(&mut self, expected: &Token) -> Result<(), QueryError> {
        match self.next() {
            Some(t) if t == *expected => Ok(()),
            Some(t) => Err(QueryError::Unexpected(format!(
                "Expected token {:?}, found {:?}", expected, t
            ))),
            None => Err(QueryError::Unexpected("Unexpected end of input".into())),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

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