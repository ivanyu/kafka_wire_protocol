use proptest::{collection, option};
use proptest::prelude::*;
use proptest::prelude::Strategy;
use crate::tagged_fields::RawTaggedField;

pub(crate) fn string() -> impl Strategy<Value=String> {
    "\"[0-9a-zA-Z]{0,10}\""
}

pub(crate) fn optional_string() -> impl Strategy<Value=Option<String>> {
    option::of(string())
}

pub(crate) fn bytes() -> impl Strategy<Value=Vec<u8>> {
    collection::vec(prop::num::u8::ANY, collection::size_range(0..10))
}

pub(crate) fn optional_bytes() -> impl Strategy<Value=Option<Vec<u8>>> {
    option::of(bytes())
}

pub(crate) fn vec<T>() -> impl Strategy<Value=Vec<T>>
where
    T: Arbitrary,
{
    collection::vec(any::<T>(), collection::size_range(0..10))
}

pub(crate) fn optional_vec<T>() -> impl Strategy<Value=Option<Vec<T>>>
where
    T: Arbitrary,
{
    option::of(vec())
}

prop_compose! {
    pub(crate) fn unknown_tagged_fields()
                                       (len in 0..=2_usize)
                                       (data in collection::vec(vec::<u8>(), 0..=len)) -> Vec<RawTaggedField> {
        data.into_iter().enumerate()
            .map(|(tag, d)| RawTaggedField { tag: tag as i32, data: d })
            .collect()
    }
}
