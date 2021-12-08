## Serialize DataTypes

```rs
Enum DataTypes {
    Int8         [0]
    Int16        [1]
    Int32        [2]
    Int64        [3]
    Uint8        [4]
    Uint16       [5]
    Uint32       [6]
    Uint64       [7]
    Float32      [8]
    Float64      [9]
    VarChar(u64) [10] (next 8 bytes are size)
}

Example: DataTypes::Float64 => [8]
Example: DataTypes::VarChar(10) => [9 0 0 0 0 0 0 0 10]
Example: DataTypes::Int8 => [0]
```

## Serialize Vec<u8>

```rs
Struct Vec<u8>

Example: [
    34 (Indicates Vec<u8>)
    0 0 0 0 0 0 0 10 (Length of the Vector)
    'h' 'e' 'l' 'l' 'o' 'w' 'o' 'r' 'l' 'd' (Data of the Vector)
]
```

## Serialize Key

```rs
Struct Key {
    datatype DataTypes
    name Vec<u8>
}
```
Example:\
    75 (Indicates Key)\
    Check [Serialize DataType](#serialize-datatypes)\
    Check [Serialize Vec<u8>](#serialize-vec<u8>) (Does not need byte 34 as identifier. Only length and data)

## Serialize