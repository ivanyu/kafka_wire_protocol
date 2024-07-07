#[cfg(test)]

use std::fmt::Debug;
use std::io::{Cursor, Seek, SeekFrom};
use proptest::prelude::TestCaseError;
use proptest::prop_assert_eq;
use crate::primitives::KafkaReadable;
use crate::primitives::KafkaWritable;

pub(crate) mod proptest_strategies;
pub(crate) mod serde_bytes;
pub(crate) mod serde_option_bytes;

pub(crate) fn test_serde<T>(data: T) -> Result<(), TestCaseError>
where
    T: KafkaReadable + KafkaWritable + Debug + PartialEq,
{
    let mut cur = Cursor::new(Vec::<u8>::new());
    data.write(&mut cur).unwrap();

    cur.seek(SeekFrom::Start(0)).unwrap();
    let data_read = T::read(&mut cur).unwrap();
    prop_assert_eq!(data_read, data);
    Ok(())
}
