// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// EndTxnRequest, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct EndTxnRequest {
    /// The ID of the transaction to end.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub transactional_id: String,
    /// The producer ID.
    pub producer_id: i64,
    /// The current epoch associated with the producer.
    pub producer_epoch: i16,
    /// True if the transaction was committed, false if it was aborted.
    pub committed: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for EndTxnRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        26
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        4
    }
}

impl Request for EndTxnRequest { }

impl Default for EndTxnRequest {
    fn default() -> Self {
        EndTxnRequest {
            transactional_id: String::from(""),
            producer_id: 0_i64,
            producer_epoch: 0_i16,
            committed: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl EndTxnRequest {
    pub fn new<S1: AsRef<str>>(transactional_id: S1, producer_id: i64, producer_epoch: i16, committed: bool) -> Self {
        Self {
            transactional_id: transactional_id.as_ref().to_string(),
            producer_id,
            producer_epoch,
            committed,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_end_txn_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = EndTxnRequest::new(
            String::from(""),
            0_i64,
            0_i16,
            false,
        );
        assert_eq!(d, EndTxnRequest::default());
    }
}

impl Readable for EndTxnRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let transactional_id = String::read_ext(input, "transactional_id", true)?;
        let producer_id = i64::read(input)?;
        let producer_epoch = i16::read(input)?;
        let committed = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(EndTxnRequest {
            transactional_id, producer_id, producer_epoch, committed, _unknown_tagged_fields
        })
    }
}

impl Writable for EndTxnRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.transactional_id.write_ext(output, "self.transactional_id", true)?;
        self.producer_id.write(output)?;
        self.producer_epoch.write(output)?;
        self.committed.write(output)?;
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
        crate::test_utils::test_java_default::<EndTxnRequest>("EndTxnRequest", 4);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: EndTxnRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: EndTxnRequest) {
            crate::test_utils::test_java_arbitrary(&data, "EndTxnRequest", 4);
        }
    }
}
