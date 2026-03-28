#![allow(dead_code)]
#![cfg(feature = "derive")]

#[derive(cracode::Encode, cracode::Decode)]
pub struct Eg<D, E> {
    data: (D, E),
}
