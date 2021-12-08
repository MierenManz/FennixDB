trait Serialize {
    fn serialize<T: From<Vec<u8>>>(&self) -> T;
}
