#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod connector;
use bincode::config::Configuration;
pub mod msg;

mod rfe;
pub use rfe::*;

pub mod reflect;

pub use macros;
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
