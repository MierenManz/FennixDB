use super::super::super::types::DataTypes;
use super::super::util::size_of;
use std::str;

pub struct Key {
    key: Vec<u8>,
    datatype: DataTypes,
    data_size_aligned: usize,
}

impl Key {
    #[inline(always)]
    pub fn new(key: String, datatype: DataTypes) -> Self {
        Self {
            key: key.into_bytes(),
            datatype,
            data_size_aligned: size_of(datatype),
        }
    }

    #[inline(always)]
    pub fn get_size(&self) -> usize {
        self.data_size_aligned
    }

    #[inline(always)]
    pub fn get_datatype(&self) -> DataTypes {
        self.datatype
    }

    #[inline(always)]
    pub fn get_name(&self) -> Result<String, str::Utf8Error> {
        str::from_utf8(&self.key).map(|s| s.to_string())
    }
}
