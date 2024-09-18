// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// EndTxnRequest, version 0.
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
}

impl ApiMessage for EndTxnRequest {
    fn api_key(&self) -> i16 {
        26
    }
    
    fn version(&self) -> i16 {
        0
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
        let transactional_id = String::read_ext(input, "transactional_id", false)?;
        let producer_id = i64::read(input)?;
        let producer_epoch = i16::read(input)?;
        let committed = bool::read(input)?;
        Ok(EndTxnRequest {
            transactional_id, producer_id, producer_epoch, committed
        })
    }
}

impl Writable for EndTxnRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.transactional_id.write_ext(output, "self.transactional_id", false)?;
        self.producer_id.write(output)?;
        self.producer_epoch.write(output)?;
        self.committed.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<EndTxnRequest>("EndTxnRequest", 0);
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
            crate::test_utils::test_java_arbitrary(&data, "EndTxnRequest", 0);
        }
    }
}
