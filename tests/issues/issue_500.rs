#![cfg(all(feature = "serde", feature = "derive", feature = "std"))]

extern crate std;

type NodeId = u64;

use std::collections::BTreeSet;

#[derive(
    cracode::Encode,
    cracode::Decode,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    Debug,
    PartialEq,
    Eq,
)]
pub struct Membership {
    /// learners set
    learners: BTreeSet<NodeId>,
}

#[test]
fn test() {
    let mut start = Membership {
        learners: BTreeSet::new(),
    };
    start.learners.insert(1);

    let config = cracode::config::legacy();
    let encoded = cracode::encode_to_vec(&start, config).unwrap();
    std::dbg!(&encoded);
    let decoded: Membership = cracode::serde::decode_from_slice(&encoded, config).unwrap().0;
    assert_eq!(start, decoded);
}
