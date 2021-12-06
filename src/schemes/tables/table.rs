use super::super::super::errors::InsertionError;
use super::key::Key;
use super::row::Row;
use super::table_head::TableHead;

pub struct Table {
    head: TableHead,
    rows: Vec<Row>,
}

impl Table {
    #[inline(always)]
    pub fn new(keys: Vec<Key>) -> Self {
        Self {
            head: TableHead::new(keys),
            rows: Vec::new(),
        }
    }

    pub fn insert(row: Row) -> Result<(), InsertionError> {
        Ok(())
    }
}
