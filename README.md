# query_compiler

Experimental tool to compile and execute modern, readable queries over CSV files, using a next-gen declarative language.

## Status

**Experimental**. This is a functional MVP. The structure and features are subject to rapid change.

## Example query

```txt
source "data/clients.csv"
show name, age
filter (age > 25 and city == "Recife") or (name == "Ana" and age < 25)
sort by age desc
cap 10
unique
```

## How to use

### 1. Build

```sh
cargo build --release
```

### 2. Run a query

```sh
cargo run -- --query example.query --show
```

### 3. Export result

```sh
cargo run -- --query example.query --output result.csv
```

The file will be saved automatically to the given path.

## Requirements

- Rust 1.74+
- Polars 0.39+ (installed via Cargo, backend for fast execution)
- [logos](https://crates.io/crates/logos) (lexer)

## Supported types

- String, Integer, Float, Boolean

- Example:
  `filter price > 10.5 and active == true and name == "Ana"`

## Supported features

- `source`: path to a CSV file
- `show`: columns to return
- `filter`: filtering using `==`, `!=`, `>`, `<`, `>=`, `<=`, `and`, `or`
- `sort by`: sorting by one column (ascending or descending)
- `cap`: limits the number of output rows
- `unique`: removes duplicates
- Optional CSV/JSON export via `--output`

## Example CSV

```csv
name,age,city,price,active
João,32,Recife,10.5,true
Maria,19,Olinda,15.0,false
José,40,Recife,8.0,true
Ana,22,Natal,23.2,true
```

## Architecture

- Lexer powered by [`logos`](https://crates.io/crates/logos)
- Hand-crafted parser builds an AST
- Execution engine uses `polars` DataFrame

## License

MIT