#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub source: String,
    pub filter: Option<Expr>,
    pub show: Vec<String>,
    pub sort: Option<Sort>,
    pub cap: Option<usize>,
    pub map: Option<(String, Expr)>,
    pub unique: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sort {
    pub column: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Eq(String, Value),
    NotEq(String, Value),
    Gt(String, Value),
    Lt(String, Value),
    Gte(String, Value),
    Lte(String, Value),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),

    // pendent
    MapExpr(Box<Expr>, String), 
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(i64),
    Float(f64),
    Bool(bool),
}


// pendent
impl SortDirection {
    pub fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "desc" => SortDirection::Desc,
            _ => SortDirection::Asc,
        }
    }
}
