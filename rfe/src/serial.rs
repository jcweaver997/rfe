use anyhow::Result;

pub trait SerialPort {
    fn open(&mut self) -> Result<()>;
    fn write(&mut self, bytes: &[u8]) -> Result<usize>;
    fn read(&mut self, bytes: &mut [u8]) -> Result<usize>;
}

#[cfg(feature = "std")]
mod serial_std {
    use std::io::{Read, Write};

    use anyhow::anyhow;
    use mio_serial::{SerialPortBuilder, SerialPortBuilderExt, SerialStream};
    pub struct StdSerialPort {
        config: SerialPortBuilder,
        serial: Option<SerialStream>,
    }

    impl StdSerialPort {
        pub fn new(config: SerialPortBuilder) -> Self {
            Self {
                config,
                serial: None,
            }
        }
    }

    impl super::SerialPort for StdSerialPort {
        fn open(&mut self) -> anyhow::Result<()> {
            self.serial = None;
            self.serial = Some(self.config.clone().open_native_async()?);
            return Ok(());
        }

        fn write(&mut self, bytes: &[u8]) -> anyhow::Result<usize> {
            Ok(self
                .serial
                .as_mut()
                .ok_or(anyhow!("port not open"))?
                .write(bytes)?)
        }

        fn read(&mut self, bytes: &mut [u8]) -> anyhow::Result<usize> {
            Ok(self
                .serial
                .as_mut()
                .ok_or(anyhow!("port not open"))?
                .read(bytes)?)
        }
    }
}

#[cfg(feature = "std")]
pub use serial_std::*;
