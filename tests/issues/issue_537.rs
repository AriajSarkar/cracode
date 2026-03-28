#![allow(dead_code)]
#![cfg(all(feature = "derive", feature = "std"))]

use cracode::{Decode, Encode};

#[derive(Encode, Decode)]
struct Foo<Bar = ()> {
    x: Bar,
}
