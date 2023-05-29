#![feature(iter_array_chunks, array_chunks)]
pub mod algorithms;
pub mod cryptobuff;
pub mod stats;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[cfg(test)]
mod set_1;
mod set_2;
