use std::error::Error;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct CreationError {
    error: String,
    name: &'static str,
}

impl CreationError {
    pub fn new<T: Into<String>>(error: T) -> Self {
        Self {
            name: "Creation Error",
            error: error.into(),
        }
    }
}

impl Display for CreationError {
    fn fmt<'a>(&self, formatter: &mut Formatter<'a>) -> Result<(), FmtError> {
        formatter.write_str(self.name)?;
        formatter.write_str("\n")?;
        formatter.write_str(&self.error)?;
        Ok(())
    }
}

impl Error for CreationError {}
