use polars::prelude::*;
use crate::ast::{Query, Expr, Value, OrderDirection};

/// Executes a parsed query against a CSV file and returns a DataFrame.
pub fn execute_query(query: Query) -> anyhow::Result<DataFrame> {
    let mut df = CsvReader::from_path(&query.from)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    if let Some(expr) = query.where_clause {
        df = apply_filter(df, expr)?;
    }

    if !query.select.is_empty() {
        df = df.select(&query.select)?;
    }

    if let Some(order) = query.order_by {
        let descending = matches!(order.direction, OrderDirection::Desc);
        let options = SortMultipleOptions {
            descending: vec![descending],
            ..Default::default()
        };
        df = df.sort(&[order.column.as_str()], options)?;
    }

    Ok(df)
}

/// Applies a boolean filter expression to a DataFrame.
fn apply_filter(df: DataFrame, expr: Expr) -> anyhow::Result<DataFrame> {
    let series = eval_expr(&df, expr)?;
    let mask = series.bool()?;
    Ok(df.filter(mask)?)
}

/// Evaluates an expression into a boolean Series.
fn eval_expr(df: &DataFrame, expr: Expr) -> anyhow::Result<Series> {
    use Expr::*;

    match expr {
        Eq(col, val) => {
            let right = value_series(&col, val)?;
            Ok(df.column(&col)?.equal(&right)?.into_series())
        }
        NotEq(col, val) => {
            let right = value_series(&col, val)?;
            Ok(df.column(&col)?.not_equal(&right)?.into_series())
        }
        Gt(col, val) => {
            let right = value_series(&col, val)?;
            Ok(df.column(&col)?.gt(&right)?.into_series())
        }
        Lt(col, val) => {
            let right = value_series(&col, val)?;
            Ok(df.column(&col)?.lt(&right)?.into_series())
        }
        Gte(col, val) => {
            let right = value_series(&col, val)?;
            Ok(df.column(&col)?.gt_eq(&right)?.into_series())
        }
        Lte(col, val) => {
            let right = value_series(&col, val)?;
            Ok(df.column(&col)?.lt_eq(&right)?.into_series())
        }
        And(left, right) => {
            let left_result = eval_expr(df, *left)?;
            let l = left_result.bool()?;

            let right_result = eval_expr(df, *right)?;
            let r = right_result.bool()?;

            Ok((l & r).into_series())
        }
        Or(left, right) => {
            let left_result = eval_expr(df, *left)?;
            let l = left_result.bool()?;

            let right_result = eval_expr(df, *right)?;
            let r = right_result.bool()?;

            Ok((l | r).into_series())
        }
    }
}

/// Converts a literal value into a Series for comparison.
fn value_series(col_name: &str, val: Value) -> anyhow::Result<Series> {
    Ok(match val {
        Value::Number(n) => Series::new(col_name, [n]),
        Value::String(s) => Series::new(col_name, [s]),
    })
}
