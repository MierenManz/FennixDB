use super::super::types::DataTypes;

pub const ALIGNMENT_SIZE: usize = 4;

#[inline(always)]
pub fn align(size: usize) -> usize {
    (size + ALIGNMENT_SIZE - 1) & !(ALIGNMENT_SIZE - 1)
}

#[inline(always)]
pub fn size_of(datatype: DataTypes) -> usize {
    match datatype {
        DataTypes::Int8 => align(1),
        DataTypes::Int16 => align(2),
        DataTypes::Int32 => align(4),
        DataTypes::Int64 => align(8),
        DataTypes::Uint8 => align(1),
        DataTypes::Uint16 => align(2),
        DataTypes::Uint32 => align(4),
        DataTypes::Uint64 => align(8),
        DataTypes::Float => align(4),
        DataTypes::Float64 => align(8),
        DataTypes::VarChar(len) => align(len),
    }
}
