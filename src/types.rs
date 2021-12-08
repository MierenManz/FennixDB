#[derive(Copy, Clone, Debug)]
pub enum DataTypes {
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float,
    Float64,
    VarChar(u64),
}
