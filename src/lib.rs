//! I2C Commands for EZO EC Chip, taken from their Datasheet.
//! This chip is used for electrical conductivity measurement. It features
//! calibration, sleep mode, scale, etc.
#![feature(str_checked_slicing)]
#![feature(exclusive_range_pattern)]

extern crate failure;
#[macro_use]
extern crate ezo_common;
extern crate i2cdev;

/// Issuable commands for the EZO EC Chip.
pub mod command;

/// Parseable responses from the EZO EC Chip.
pub mod response;

// Re-export errors from ezo_common crate.
pub use ezo_common::errors::{ErrorKind, EzoError};
