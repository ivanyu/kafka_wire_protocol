// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Data};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// KRaftVersionRecord, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct KRaftVersionRecord {
    /// The version of the kraft version record.
    pub version: i16,
    /// The kraft protocol version.
    pub kraft_version: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for KRaftVersionRecord {
    fn api_key(&self) -> i16 {
        -1
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Data for KRaftVersionRecord { }

impl Default for KRaftVersionRecord {
    fn default() -> Self {
        KRaftVersionRecord {
            version: 0_i16,
            kraft_version: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl KRaftVersionRecord {
    pub fn new(version: i16, kraft_version: i16) -> Self {
        Self {
            version,
            kraft_version,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_kraft_version_record_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = KRaftVersionRecord::new(
            0_i16,
            0_i16,
        );
        assert_eq!(d, KRaftVersionRecord::default());
    }
}

impl Readable for KRaftVersionRecord {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let version = i16::read(input)?;
        let kraft_version = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(KRaftVersionRecord {
            version, kraft_version, _unknown_tagged_fields
        })
    }
}

impl Writable for KRaftVersionRecord {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.version.write(output)?;
        self.kraft_version.write(output)?;
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
        crate::test_utils::test_java_default::<KRaftVersionRecord>("KRaftVersionRecord", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: KRaftVersionRecord) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: KRaftVersionRecord) {
            crate::test_utils::test_java_arbitrary(&data, "KRaftVersionRecord", 0);
        }
    }
}
