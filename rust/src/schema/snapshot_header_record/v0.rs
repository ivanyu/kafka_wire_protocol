// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Data};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// SnapshotHeaderRecord, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct SnapshotHeaderRecord {
    /// The version of the snapshot header record
    pub version: i16,
    /// The append time of the last record from the log contained in this snapshot
    pub last_contained_log_timestamp: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for SnapshotHeaderRecord {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        -1
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Data for SnapshotHeaderRecord { }

impl Default for SnapshotHeaderRecord {
    fn default() -> Self {
        SnapshotHeaderRecord {
            version: 0_i16,
            last_contained_log_timestamp: 0_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl SnapshotHeaderRecord {
    pub fn new(version: i16, last_contained_log_timestamp: i64) -> Self {
        Self {
            version,
            last_contained_log_timestamp,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_snapshot_header_record_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = SnapshotHeaderRecord::new(
            0_i16,
            0_i64,
        );
        assert_eq!(d, SnapshotHeaderRecord::default());
    }
}

impl Readable for SnapshotHeaderRecord {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let version = i16::read(input)?;
        let last_contained_log_timestamp = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(SnapshotHeaderRecord {
            version, last_contained_log_timestamp, _unknown_tagged_fields
        })
    }
}

impl Writable for SnapshotHeaderRecord {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.version.write(output)?;
        self.last_contained_log_timestamp.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<SnapshotHeaderRecord>("SnapshotHeaderRecord", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: SnapshotHeaderRecord) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: SnapshotHeaderRecord) {
            crate::test_utils::test_java_arbitrary(&data, "SnapshotHeaderRecord", 0);
        }
    }
}
