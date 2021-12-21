use super::super::super::super::errors::InsertionError;
use super::key::Key;
use super::row::Row;

pub struct Table {
    row_ptrs: Vec<usize>,
    rows: Vec<Row>,
    collumn_keys: Vec<Key>,
}

impl Table {
    #[inline(always)]
    pub fn new(keys: Vec<Key>) -> Self {
        Self {
            collumn_keys: keys,
            row_ptrs: Vec::new(),
            rows: Vec::new(),
        }
    }

    pub fn insert(row: Row) -> Result<(), InsertionError> {
        Ok(())
    }
}
