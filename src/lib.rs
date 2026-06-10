#![cfg_attr(not(test), no_std)]
#![deny(unsafe_code)]

pub mod driver;
pub mod error;
pub mod packets;

pub(crate) type Result<T> = core::result::Result<T, crate::error::Error>;

pub use pixy2::Pixy2;

use crate::driver::pixy2;
