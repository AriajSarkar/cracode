#![cfg(all(feature = "std", feature = "derive"))]

extern crate std;

use std::collections::BTreeMap;

#[derive(cracode::Decode, cracode::Encode)]
struct AllTypes(BTreeMap<u8, AllTypes>);

#[test]
fn test_issue_467() {
    let _result: Result<(AllTypes, _), _> =
        cracode::decode_from_slice(&[], cracode::config::standard().with_limit::<1024>());
}
