use super::super::types::DataTypes;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct InsertionError {
    error: String,
    name: &'static str,
    table: String,
    key: String,
    value: String,
    datatype: DataTypes,
}

impl InsertionError {
    pub fn new(
        error: String,
        table: String,
        key: String,
        value: String,
        datatype: DataTypes,
    ) -> Self {
        Self {
            name: "Insertion Error",
            error,
            table,
            key,
            value,
            datatype,
        }
    }
}

impl Display for InsertionError {
    fn fmt<'a>(&self, formatter: &mut Formatter<'a>) -> Result<(), FmtError> {
        formatter.write_str(self.name)?;
        formatter.write_str("\n")?;
        formatter.write_str(&self.error)?;
        formatter.write_fmt(format_args!(
            "Debug Information:\nTable: {}\nKey: {}\nValue: {}",
            self.table, self.key, self.value
        ))?;
        Ok(())
    }
}

impl Error for InsertionError {}
