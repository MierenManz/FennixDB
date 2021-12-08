use super::key::Key;

pub struct TableHead {
    row_size: usize,
    collumn_keys: Vec<Key>,
}

impl TableHead {
    #[inline(always)]
    pub fn new(keys: Vec<Key>) -> Self {
        let mut total_size: usize = 0;
        for key in &keys {
            total_size += key.get_size();
        }

        Self {
            row_size: total_size,
            collumn_keys: keys,
        }
    }

    #[inline(always)]
    pub fn get_row_size<T: From<usize>>(&self) -> T {
        self.row_size.into()
    }

    #[inline(always)]
    pub fn get_collumn_keys(&self) -> &[Key] {
        &self.collumn_keys
    }
}
