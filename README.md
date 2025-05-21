# query_compiler

Experimental tool to compile and execute simple queries over CSV files (for now), using a declarative SQL-like language.

## Status

**Experimental**. This is a functional MVP. The structure is subject to change.

## Example query

```
from "data/clients.csv"
select name, age
where age > 25 and city == "Recife"
order by age desc
```

## How to use

### 1. Build

```
cargo build --release
```

### 2. Run

```
cargo run -- example.query
```

### 3. Export result

```
cargo run -- example.query --output result.csv
```

The file will be saved automatically to `reports/result.csv`.

## Requirements

- Rust 1.74+
- Polars (via Cargo) as the execution backend

## Features

- `from`: path to a CSV file
- `select`: columns to return
- `where`: filtering using `==`, `!=`, `>`, `<`, `>=`, `<=`, `and`, `or`
- `order by`: sorting by one column (ascending or descending)
- Optional CSV export via `--output`

## Example CSV

```
name,age,city
João,32,Recife
Maria,19,Olinda
José,40,Recife
Ana,22,Natal
```

## Architecture

- Lexer powered by [`logos`](https://crates.io/crates/logos)
- Manual parser that builds an AST
- Execution engine using `polars` DataFrame

## License

MIT