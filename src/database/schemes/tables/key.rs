use super::super::super::super::types::DataTypes;

#[repr(C)]
pub struct Key {
    datatype: DataTypes,
    name: String,
}

impl Key {
    #[inline(always)]
    pub fn new(name: String, datatype: DataTypes) -> Self {
        Self {
            name,
            datatype,
        }
    }

    #[inline(always)]
    pub fn get_data_size(&self) -> usize {
        let mut datatype_size = 1;
        if matches!(self.datatype, DataTypes::VarChar(_)) {
            datatype_size = 9;
        }

        datatype_size
    }

    #[inline(always)]
    pub fn get_datatype(&self) -> DataTypes {
        self.datatype
    }

    #[inline(always)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
