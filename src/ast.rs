/// Represents a parsed query from a `.query` file.
#[derive(Debug)]
pub struct Query {
    pub from: String,
    pub select: Vec<String>,
    pub where_clause: Option<Expr>,
    pub order_by: Option<Order>,
}

/// Represents an `order by` clause.
#[derive(Debug)]
pub struct Order {
    pub column: String,
    pub direction: OrderDirection,
}

/// Direction used in `order by`.
#[derive(Debug, PartialEq)]
pub enum OrderDirection {
    Asc,
    Desc,
}

/// Expression used in `where` filters.
#[derive(Debug)]
pub enum Expr {
    Eq(String, Value),
    NotEq(String, Value),
    Gt(String, Value),
    Lt(String, Value),
    Gte(String, Value),
    Lte(String, Value),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

/// Literal values used in expressions.
#[derive(Debug)]
pub enum Value {
    String(String),
    Number(i64),
}
