#![cfg(all(feature = "serde", feature = "std"))]

use glam::vec3;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Instance {
    position: glam::Vec3,
}

#[test]
fn test() {
    let instance = Instance {
        position: vec3(2.0, 2.0, 2.0),
    };

    let m = cracode::serde::encode_to_vec(&instance, cracode::config::standard()).unwrap();
    let instance2: Instance =
        cracode::serde::decode_from_slice(&m, cracode::config::standard()).unwrap().0;

    assert_eq!(instance, instance2);
}
