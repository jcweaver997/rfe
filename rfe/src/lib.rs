#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod connector;
use bincode::config::Configuration;
pub mod msg;

mod rfe;
pub use rfe::*;

#[cfg(feature = "to_csv")]
mod to_csv;
#[cfg(feature = "to_csv")]
pub use to_csv::*;

pub mod time;
pub mod utils;

pub const BINCODE_CONFIG: Configuration = bincode::config::standard();

#[macro_export]
macro_rules! unwrap_print_err {
    ($x:expr, $msg: tt) => {
        if let Err(_) = $x {
            error!($msg)
        }
    };
}
