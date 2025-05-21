# query_compiler

Experimental tool to compile and execute modern, readable queries over CSV files, using a next-gen declarative language.

## Status

**Experimental**. This is a functional MVP. The structure and features are subject to rapid change.

## Example query

```txt
source "data/clients.csv"
show name, age
filter age > 25 and city == "Recife"
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
name,age,city
João,32,Recife
Maria,19,Olinda
José,40,Recife
Ana,22,Natal
```

## Architecture

- Lexer powered by [`logos`](https://crates.io/crates/logos)
- Hand-crafted parser builds an AST
- Execution engine uses `polars` DataFrame

## License

MIT