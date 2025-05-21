use crate::ast::{Query, Expr, Value, SortDirection};
use polars::prelude::*;
use anyhow::Result;

pub fn execute_query(query: Query) -> Result<DataFrame> {
    let mut df = CsvReader::from_path(&query.source)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    if let Some(expr) = query.filter {
        df = apply_filter(df, expr)?;
    }

    if !query.show.is_empty() {
        df = df.select(&query.show)?;
    }

    if let Some((field, expr)) = query.map {
        let mut df2 = df.clone();
        let mut new_col = eval_map_expr(&df, expr)?;
        new_col.rename(&field);
        df2.with_column(new_col)?;
        df = df2;
    }

    if query.unique {
        df = df.unique(None, UniqueKeepStrategy::First, None)?;
    }

    if let Some(sort) = query.sort {
        let descending = matches!(sort.direction, SortDirection::Desc);
        let options = SortMultipleOptions {
            descending: vec![descending],
            ..Default::default()
        };
        df = df.sort(&[sort.column.as_str()], options)?;
    }

    if let Some(cap) = query.cap {
        let h = df.height();
        let cap = cap.min(h);
        df = df.head(Some(cap));
    }

    Ok(df)
}

fn apply_filter(df: DataFrame, expr: Expr) -> Result<DataFrame> {
    let mask = eval_expr(&df, expr)?.bool()?.clone();
    Ok(df.filter(&mask)?)
}

fn eval_expr(df: &DataFrame, expr: Expr) -> Result<Series> {
    use Expr::*;
    match expr {
        Eq(col, val) => Ok(df.column(&col)?.equal(&value_series(df, &col, val)?)?.into_series()),
        NotEq(col, val) => Ok(df.column(&col)?.not_equal(&value_series(df, &col, val)?)?.into_series()),
        Gt(col, val) => Ok(df.column(&col)?.gt(&value_series(df, &col, val)?)?.into_series()),
        Lt(col, val) => Ok(df.column(&col)?.lt(&value_series(df, &col, val)?)?.into_series()),
        Gte(col, val) => Ok(df.column(&col)?.gt_eq(&value_series(df, &col, val)?)?.into_series()),
        Lte(col, val) => Ok(df.column(&col)?.lt_eq(&value_series(df, &col, val)?)?.into_series()),
        And(lhs, rhs) => {
            let l = eval_expr(df, *lhs)?.bool()?.clone();
            let r = eval_expr(df, *rhs)?.bool()?.clone();
            Ok((l & r).into_series())
        }
        Or(lhs, rhs) => {
            let l = eval_expr(df, *lhs)?.bool()?.clone();
            let r = eval_expr(df, *rhs)?.bool()?.clone();
            Ok((l | r).into_series())
        }
        MapExpr(_, _) => Err(anyhow::anyhow!("MapExpr is not allowed in filter")),
    }
}

fn value_series(_df: &DataFrame, col: &str, val: Value) -> Result<Series> {
    Ok(match val {
        Value::Number(n) => Series::new(col, vec![n]),
        Value::Float(f) => Series::new(col, vec![f]),
        Value::String(s) => Series::new(col, vec![s]),
        Value::Bool(b) => Series::new(col, vec![b]),
    })
}

fn eval_map_expr(df: &DataFrame, expr: Expr) -> Result<Series> {
    match expr {
        Expr::Eq(field, value) => Ok(df.column(&field)?.equal(&value_series(df, &field, value)?)?.into_series()),
        Expr::Gt(field, value) => Ok(df.column(&field)?.gt(&value_series(df, &field, value)?)?.into_series()),
        Expr::Lt(field, value) => Ok(df.column(&field)?.lt(&value_series(df, &field, value)?)?.into_series()),
        Expr::And(lhs, rhs) => {
            let l = eval_map_expr(df, *lhs)?.bool()?.clone();
            let r = eval_map_expr(df, *rhs)?.bool()?.clone();
            Ok((l & r).into_series())
        }
        Expr::Or(lhs, rhs) => {
            let l = eval_map_expr(df, *lhs)?.bool()?.clone();
            let r = eval_map_expr(df, *rhs)?.bool()?.clone();
            Ok((l | r).into_series())
        }
        Expr::NotEq(_, _) | Expr::Gte(_, _) | Expr::Lte(_, _) | Expr::MapExpr(_, _) => {
            Err(anyhow::anyhow!("Map expressions not fully supported yet"))
        }
    }
}
