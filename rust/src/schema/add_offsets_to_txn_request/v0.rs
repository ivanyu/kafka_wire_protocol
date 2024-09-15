// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AddOffsetsToTxnRequest {
    /// The transactional id corresponding to the transaction.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub transactional_id: String,
    /// Current producer id in use by the transactional id.
    pub producer_id: i64,
    /// Current epoch associated with the producer id.
    pub producer_epoch: i16,
    /// The unique group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
}

impl ApiMessage for AddOffsetsToTxnRequest {
    fn api_key(&self) -> i16 {
        25
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for AddOffsetsToTxnRequest { }

impl Default for AddOffsetsToTxnRequest {
    fn default() -> Self {
        AddOffsetsToTxnRequest {
            transactional_id: String::from(""),
            producer_id: 0_i64,
            producer_epoch: 0_i16,
            group_id: String::from(""),
        }
    }
}

impl AddOffsetsToTxnRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(transactional_id: S1, producer_id: i64, producer_epoch: i16, group_id: S2) -> Self {
        Self {
            transactional_id: transactional_id.as_ref().to_string(),
            producer_id,
            producer_epoch,
            group_id: group_id.as_ref().to_string(),
        }
    }
}

#[cfg(test)]
mod tests_add_offsets_to_txn_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AddOffsetsToTxnRequest::new(
            String::from(""),
            0_i64,
            0_i16,
            String::from(""),
        );
        assert_eq!(d, AddOffsetsToTxnRequest::default());
    }
}

impl Readable for AddOffsetsToTxnRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let transactional_id = String::read_ext(input, "transactional_id", false)?;
        let producer_id = i64::read(input)?;
        let producer_epoch = i16::read(input)?;
        let group_id = String::read_ext(input, "group_id", false)?;
        Ok(AddOffsetsToTxnRequest {
            transactional_id, producer_id, producer_epoch, group_id
        })
    }
}

impl Writable for AddOffsetsToTxnRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.transactional_id.write_ext(output, "self.transactional_id", false)?;
        self.producer_id.write(output)?;
        self.producer_epoch.write(output)?;
        self.group_id.write_ext(output, "self.group_id", false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<AddOffsetsToTxnRequest>("AddOffsetsToTxnRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AddOffsetsToTxnRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AddOffsetsToTxnRequest) {
            crate::test_utils::test_java_arbitrary(&data, "AddOffsetsToTxnRequest", 0);
        }
    }
}
