use alloc::string::String;
use anyhow::Result;

#[derive(Debug)]
pub struct DsFile {
    pub dir: String,
}

impl DsFile {
    pub fn new(dir: String) -> Self {
        Self { dir }
    }

    pub fn close(&mut self) {}

    pub fn open(&mut self) {}

    pub fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        return Ok(0);
    }

    pub fn flush(&mut self) -> Result<()> {
        return Ok(());
    }
}
