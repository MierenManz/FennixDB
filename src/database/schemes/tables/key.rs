use super::super::super::super::types::DataTypes;
use std::str;

#[repr(C)]
pub struct Key {
    datatype: DataTypes,
    name: Vec<u8>,
}

impl Key {
    #[inline(always)]
    pub fn new(name: String, datatype: DataTypes) -> Self {
        Self {
            name: name.into_bytes(),
            datatype,
        }
    }

    #[inline(always)]
    pub fn get_size(&self) -> usize {
        let mut datatype_size = 1;
        if matches!(self.datatype, DataTypes::VarChar(_)) {
            datatype_size = 9;
        }

        self.name.len() + datatype_size
    }

    #[inline(always)]
    pub fn get_datatype(&self) -> DataTypes {
        self.datatype
    }

    #[inline(always)]
    pub fn get_name(&self) -> Result<String, str::Utf8Error> {
        str::from_utf8(&self.name).map(|s| s.to_string())
    }
}
