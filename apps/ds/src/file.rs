extern crate alloc;
use alloc::string::String;
use anyhow::Result;

pub trait DsFile: Default {
    fn new(dir: String) -> Self;
    fn close(&mut self);
    fn open(&mut self);
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
}

#[cfg(feature = "std")]
mod file_std {
    extern crate std;

    use alloc::{
        format,
        string::{String, ToString},
    };
    use anyhow::Result;
    use chrono::Utc;
    use log::*;
    use std::{fs::File, io::Write, path::Path};

    use super::DsFile;

    #[derive(Debug, Default)]
    pub struct StdDsFile {
        pub dir: String,
        pub file: Option<File>,
        prefix: String,
    }

    impl StdDsFile {
        fn get_time(prefix: &str) -> String {
            let date = Utc::now();
            format!("{}_{}", prefix, date.format("%Y-%m-%d_%H-%M-%S.dat"))
        }
    }

    impl DsFile for StdDsFile {
        fn new(dir: String) -> Self {
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

        fn close(&mut self) {
            self.file = None;
        }

        fn open(&mut self) {
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

        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            if self.file.is_none() {
                self.open();
            }

            if let Some(f) = &mut self.file {
                return Ok(f.write(buf)?);
            } else {
                return Ok(0);
            }
        }

        fn flush(&mut self) -> Result<()> {
            if let Some(f) = &mut self.file {
                return Ok(f.flush()?);
            } else {
                return Ok(());
            }
        }
    }
}

#[cfg(feature = "std")]
pub use file_std::*;
