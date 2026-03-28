#![allow(dead_code)]
#![cfg(all(feature = "derive", feature = "std"))]

use cracode::{Decode, Encode};

#[derive(Encode, Decode)]
pub enum TypeOfFile {
    Unknown = -1,
}
