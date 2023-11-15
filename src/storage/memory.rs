// src/storage/memory.rs
use std::collections::HashMap;
use crate::types::datatype::DataType;
use crate::types::row::Row; 

struct InMemoryStorage {
    tables: HashMap<String, Table>,
}

struct Table {
    rows: Vec<Row>,
}

struct Row {
    values: Vec<DataType>,
}
