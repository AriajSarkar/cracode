#![cfg(all(feature = "std", feature = "derive"))]

extern crate std;

use std::collections::BTreeMap;

#[derive(cracode::Encode)]
struct AllTypes(BTreeMap<u8, AllTypes>);

#[test]
fn test_issue_459() {
    let _result = cracode::encode_to_vec(AllTypes(BTreeMap::new()), cracode::config::standard());
}
