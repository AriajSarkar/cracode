#![cfg(all(feature = "std", feature = "derive"))]

extern crate std;

use cracode::{Decode, Encode};
use std::borrow::Cow;
use std::string::String;

#[derive(Decode, Encode, PartialEq, Debug)]
#[cracode(
    decode_context = "()",
    borrow_decode_bounds = "&'__de U<'a, A>: ::cracode::de::BorrowDecode<'__de, ()> + '__de, '__de: 'a"
)]
struct T<'a, A: Clone + Encode + Decode<()>> {
    t: Cow<'a, U<'a, A>>,
}

#[derive(Clone, Decode, Encode, PartialEq, Debug)]
#[cracode(
    decode_context = "()",
    borrow_decode_bounds = "&'__de A: ::cracode::de::BorrowDecode<'__de, ()> + '__de, '__de: 'a"
)]
struct U<'a, A: Clone + Encode + Decode<()>> {
    u: Cow<'a, A>,
}

#[test]
fn test() {
    let u = U {
        u: Cow::Owned(String::from("Hello world")),
    };
    let t = T {
        t: Cow::Borrowed(&u),
    };
    let vec = cracode::encode_to_vec(&t, cracode::config::standard()).unwrap();

    let (decoded, len): (T<String>, usize) =
        cracode::decode_from_slice(&vec, cracode::config::standard()).unwrap();

    assert_eq!(t, decoded);
    assert_eq!(len, 12);
}
