#[cfg(test)]

use std::fmt::Debug;
use std::io::{Cursor, Seek, SeekFrom};
use proptest::prelude::{ProptestConfig, TestCaseError};
use proptest::prop_assert_eq;
use proptest::test_runner::Config;
use serde::Serialize;
use static_init::dynamic;
use crate::primitives::KafkaReadable;
use crate::primitives::KafkaWritable;
use crate::test_utils::java_tester::JavaTester;

pub(crate) mod proptest_strategies;
pub(crate) mod serde_bytes;
pub(crate) mod serde_option_bytes;
pub(crate) mod java_tester;

pub(crate) fn test_serde<T>(data: &T) -> Result<(), TestCaseError>
where
    T: KafkaReadable + KafkaWritable + Debug + PartialEq + Clone,
{
    let mut cur = Cursor::new(Vec::<u8>::new());
    data.write(&mut cur).unwrap();

    cur.seek(SeekFrom::Start(0)).unwrap();
    let data_read = T::read(&mut cur).unwrap();
    prop_assert_eq!(data_read, data.clone());
    Ok(())
}

#[dynamic(drop)]
static mut JAVA_TESTER: JavaTester = JavaTester::new();

pub(crate) fn test_java<T>(data: &T, class: &str, version: u16)
where
    T: KafkaReadable + KafkaWritable + Serialize + Debug + PartialEq + Clone,
{
    let json = serde_json::to_value(&data).unwrap();
    let mut cur = Cursor::new(Vec::<u8>::new());
    data.write(&mut cur).unwrap();
    let vec = cur.into_inner();
    {
        let mut lock = JAVA_TESTER.write();
        lock.test(class, version, json, &vec);
    }
}
