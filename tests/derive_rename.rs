#![cfg(feature = "derive")]

extern crate cracode as cracode_new;

// Make sure that the `cracode` crate exists, just symlink it to `core.
extern crate core as cracode;

#[derive(cracode_new::Encode)]
#[cracode(crate = "cracode_new")]
#[allow(dead_code)]
struct DeriveRenameTest {
    a: u32,
    b: u32,
}
