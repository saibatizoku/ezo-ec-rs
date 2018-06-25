//! I2C Commands for EC EZO Chip, taken from their Datasheet.
//! This chip is used for electrical conductivity measurement. It features
//! calibration, sleep mode, scale, etc.

#![feature(str_checked_slicing)]

#![feature(exclusive_range_pattern)]

#![feature(inclusive_range_syntax)]

#![recursion_limit = "1024"]

#[macro_use]
extern crate failure;

#[macro_use]
extern crate ezo_common;

extern crate i2cdev;

// Use error-chain.
pub mod errors;

pub mod command;
pub mod response;
