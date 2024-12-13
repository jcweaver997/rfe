extern crate std;

use alloc::{
    format,
    string::{String, ToString},
};
use chrono::Utc;
use log::*;
use std::{fs::File, io::Write, path::Path};

#[derive(Debug)]
pub struct DsFile {
    pub dir: String,
    pub file: Option<File>,
    prefix: String,
}

impl DsFile {
    pub fn new(dir: String) -> Self {
        Self {
            file: None,
            prefix: dir
                .clone()
                .split("/")
                .last()
                .unwrap_or("unnamed")
                .to_string(),
            dir,
        }
    }

    pub fn close(&mut self) {
        self.file = None;
    }

    pub fn open(&mut self) {
        let time = Self::get_time(&self.prefix);
        let file_path = Path::new(&self.dir).join(time);
        self.file = match File::create(&file_path) {
            Ok(f) => Some(f),
            Err(e) => {
                error!("failed to create file at {:?} {}", file_path, e);
                None
            }
        };
    }

    fn get_time(prefix: &str) -> String {
        let date = Utc::now();
        format!("{}_{}", prefix, date.format("%Y-%m-%d_%H-%M-%S.dat"))
    }

    pub fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.file.is_none() {
            self.open();
        }

        if let Some(f) = &mut self.file {
            return f.write(buf);
        } else {
            return Ok(0);
        }
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        if let Some(f) = &mut self.file {
            return f.flush();
        } else {
            return Ok(());
        }
    }
}
