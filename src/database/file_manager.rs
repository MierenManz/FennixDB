use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

pub struct FileManager {
    file_handle: File,
}

impl FileManager {
    pub fn new(file_handle: File) -> Self {
        Self { file_handle }
    }

    pub fn write<'a, T: Into<Vec<u8>>>(&mut self, data: T, offset: SeekFrom) -> Result<()> {
        self.file_handle.seek(offset)?;
        self.file_handle.write_all(&data.into())?;
        self.file_handle.flush()?;

        Ok(())
    }

    pub fn read<T: From<Vec<u8>>>(&mut self, offset: u64, length: usize) -> Result<T> {
        let mut buffer: Vec<u8> = Vec::with_capacity(length);

        self.file_handle.seek(SeekFrom::Start(offset))?;
        self.file_handle.read_exact(&mut buffer)?;

        Ok(buffer.into())
    }

    pub fn move_data(&mut self, offset: u64, length: u64) -> Result<()> {
        let mut temp_buff: Vec<u8> = Vec::new();
        self.file_handle.seek(SeekFrom::Start(offset))?;
        self.file_handle.read_to_end(&mut temp_buff)?;
        self.file_handle.seek(SeekFrom::Start(offset + length))?;
        self.file_handle.write_all(&temp_buff)?;

        Ok(())
    }
}
