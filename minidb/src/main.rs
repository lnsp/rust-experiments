use std::collections::BTreeMap;
use std::io::{self, Stdout, Write};

struct Tuple {
    values: Vec<Value>,
}

enum Value {
    NULL(),
    INTEGER(i64),
    REAL(f64),
    TEXT(String),
    BLOB(Vec<u8>),
}

struct Column {
    id: u32,
    name: String,
    kind: Value,
}

struct Index {
    column_id: u32,
    map: BTreeMap<Value, u32>,
}

struct Table {
    name: String,
    columns: Vec<Column>,
    tuples: Vec<Tuple>,
    indices: Vec<Index>,
}

struct Database {
    tables: Vec<Table>,
}

fn handle_command() -> Result<(), io::Error> {
    let mut command = String::new();
    io::stdout().write_all(b"> ")?;
    io::stdout().flush()?;
    let bytes_read = io::stdin().read_line(&mut command)?;
    println!("read {} bytes", bytes_read);

    Ok(())
}

fn main() {
    loop {
        handle_command().expect("could not handle command")
    }
}
