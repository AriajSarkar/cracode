#![allow(clippy::disallowed_names)]
#![cfg(feature = "alloc")]

extern crate alloc;

mod utils;

use alloc::borrow::Cow;
use alloc::collections::*;
#[cfg(not(feature = "serde"))]
use alloc::rc::Rc;
#[cfg(all(target_has_atomic = "ptr", not(feature = "serde")))]
use alloc::sync::Arc;
use utils::{the_same, the_same_with_comparer};

struct Foo {
    pub a: u32,
    pub b: u32,
}

impl cracode::Encode for Foo {
    fn encode<E: cracode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), cracode::error::EncodeError> {
        self.a.encode(encoder)?;
        self.b.encode(encoder)?;
        Ok(())
    }
}

impl<Context> cracode::Decode<Context> for Foo {
    fn decode<D: cracode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, cracode::error::DecodeError> {
        Ok(Self {
            a: cracode::Decode::decode(decoder)?,
            b: cracode::Decode::decode(decoder)?,
        })
    }
}
cracode::impl_borrow_decode!(Foo);

#[test]
fn test_vec() {
    let vec = cracode::encode_to_vec(
        Foo {
            a: 5,
            b: 10,
        },
        cracode::config::standard(),
    )
    .unwrap();
    assert_eq!(vec, &[5, 10]);

    let (foo, len): (Foo, usize) =
        cracode::decode_from_slice(&vec, cracode::config::standard()).unwrap();
    assert_eq!(foo.a, 5);
    assert_eq!(foo.b, 10);
    assert_eq!(len, 2);

    let vec: Vec<u8> = cracode::decode_from_slice(
        &[4, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4],
        cracode::config::legacy(),
    )
    .unwrap()
    .0;
    assert_eq!(vec, &[1, 2, 3, 4]);

    let vec: Vec<Cow<'static, u8>> = cracode::decode_from_slice(
        &[4, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4],
        cracode::config::legacy(),
    )
    .unwrap()
    .0;
    assert_eq!(vec, &[Cow::Borrowed(&1), Cow::Borrowed(&2), Cow::Borrowed(&3), Cow::Borrowed(&4)]);
}

#[test]
fn test_alloc_commons() {
    the_same::<Vec<u32>>(vec![1, 2, 3, 4, 5]);
    the_same(String::from("Hello world"));
    the_same(Box::<u32>::new(5));
    the_same(Box::<[u32]>::from(vec![1, 2, 3, 4, 5]));
    the_same(Cow::<u32>::Owned(5));
    the_same(Cow::<u32>::Borrowed(&5));
    #[cfg(not(feature = "serde"))]
    {
        // Serde doesn't support Rc or Arc
        the_same(Rc::<u32>::new(5));
        the_same(Rc::<[u32]>::from(vec![1, 2, 3, 4, 5]));

        #[cfg(target_has_atomic = "ptr")]
        {
            the_same(Arc::<u32>::new(5));
            the_same(Arc::<[u32]>::from(vec![1, 2, 3, 4, 5]));
        }
    }

    the_same_with_comparer(
        {
            let mut map = BinaryHeap::<u32>::new();
            map.push(1);
            map.push(2);
            map.push(3);
            map.push(4);
            map.push(5);
            map
        },
        |a, b| a.iter().collect::<Vec<_>>() == b.iter().collect::<Vec<_>>(),
    );
    the_same({
        let mut map = BTreeMap::<u32, i32>::new();
        map.insert(5, -5);
        map
    });
    the_same({
        let mut set = BTreeSet::<u32>::new();
        set.insert(5);
        set
    });
    the_same({
        let mut set = VecDeque::<u32>::new();
        set.push_back(15);
        set.push_front(5);
        set
    });
}

#[test]
fn test_container_limits() {
    use cracode::{error::DecodeError, BorrowDecode, Decode};

    const DECODE_LIMIT: usize = 100_000;

    // for this test we'll create a malformed package of a lot of bytes
    let test_cases = &[
        // u64::max_value(), should overflow
        #[cfg(target_pointer_width = "64")]
        cracode::encode_to_vec(u64::MAX, cracode::config::standard()).unwrap(),
        #[cfg(target_pointer_width = "32")]
        cracode::encode_to_vec(u32::MAX, cracode::config::standard()).unwrap(),
        // A high value which doesn't overflow, but exceeds the decode limit
        cracode::encode_to_vec(DECODE_LIMIT as u64, cracode::config::standard()).unwrap(),
    ];

    fn validate_fail<T: Decode<()> + for<'de> BorrowDecode<'de, ()> + core::fmt::Debug>(
        slice: &[u8],
    ) {
        let result = cracode::decode_from_slice::<T, _>(
            slice,
            cracode::config::standard().with_limit::<DECODE_LIMIT>(),
        );

        let name = core::any::type_name::<T>();
        match result {
            Ok(_) => panic!("Decoding {} should fail, it instead succeeded", name),
            Err(DecodeError::OutsideUsizeRange(_)) if cfg!(target_pointer_width = "32") => {},
            Err(DecodeError::LimitExceeded) => {},
            Err(e) => panic!("Expected OutsideUsizeRange (on 32 bit platforms) or LimitExceeded whilst decoding {}, got {:?}", name, e),
        }
    }

    for slice in test_cases {
        validate_fail::<BinaryHeap<i32>>(slice);
        validate_fail::<BTreeMap<i32, i32>>(slice);
        validate_fail::<BTreeSet<i32>>(slice);
        validate_fail::<VecDeque<i32>>(slice);
        validate_fail::<Vec<i32>>(slice);
        validate_fail::<String>(slice);
        #[cfg(feature = "std")]
        {
            validate_fail::<std::collections::HashMap<i32, i32>>(slice);
            validate_fail::<std::collections::HashSet<i32>>(slice);
        }
    }
}

#[cfg(target_has_atomic = "ptr")]
#[test]
fn test_arc_str() {
    use alloc::sync::Arc;

    let start: Arc<str> = Arc::from("Example String");
    let mut target = [0u8; 100];
    let config = cracode::config::standard();

    let len = {
        let start: Arc<str> = Arc::clone(&start);
        cracode::encode_into_slice(start, &mut target, config).unwrap()
    };
    let slice = &target[..len];

    let decoded: Arc<str> = cracode::borrow_decode_from_slice(slice, config).unwrap().0;
    assert_eq!(decoded, start);
}
