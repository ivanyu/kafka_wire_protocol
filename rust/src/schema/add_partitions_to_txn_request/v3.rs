// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// AddPartitionsToTxnRequest, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AddPartitionsToTxnRequest {
    /// The transactional id corresponding to the transaction.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub v3_and_below_transactional_id: String,
    /// Current producer id in use by the transactional id.
    pub v3_and_below_producer_id: i64,
    /// Current epoch associated with the producer id.
    pub v3_and_below_producer_epoch: i16,
    /// The partitions to add to the transaction.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub v3_and_below_topics: Vec<AddPartitionsToTxnTopic>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AddPartitionsToTxnRequest {
    fn api_key(&self) -> i16 {
        24
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Request for AddPartitionsToTxnRequest { }

impl Default for AddPartitionsToTxnRequest {
    fn default() -> Self {
        AddPartitionsToTxnRequest {
            v3_and_below_transactional_id: String::from(""),
            v3_and_below_producer_id: 0_i64,
            v3_and_below_producer_epoch: 0_i16,
            v3_and_below_topics: Vec::<AddPartitionsToTxnTopic>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AddPartitionsToTxnRequest {
    pub fn new<S1: AsRef<str>>(v3_and_below_transactional_id: S1, v3_and_below_producer_id: i64, v3_and_below_producer_epoch: i16, v3_and_below_topics: Vec<AddPartitionsToTxnTopic>) -> Self {
        Self {
            v3_and_below_transactional_id: v3_and_below_transactional_id.as_ref().to_string(),
            v3_and_below_producer_id,
            v3_and_below_producer_epoch,
            v3_and_below_topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_add_partitions_to_txn_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AddPartitionsToTxnRequest::new(
            String::from(""),
            0_i64,
            0_i16,
            Vec::<AddPartitionsToTxnTopic>::new(),
        );
        assert_eq!(d, AddPartitionsToTxnRequest::default());
    }
}

impl Readable for AddPartitionsToTxnRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let v3_and_below_transactional_id = String::read_ext(input, "v3_and_below_transactional_id", true)?;
        let v3_and_below_producer_id = i64::read(input)?;
        let v3_and_below_producer_epoch = i16::read(input)?;
        let v3_and_below_topics = read_array::<AddPartitionsToTxnTopic>(input, "v3_and_below_topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AddPartitionsToTxnRequest {
            v3_and_below_transactional_id, v3_and_below_producer_id, v3_and_below_producer_epoch, v3_and_below_topics, _unknown_tagged_fields
        })
    }
}

impl Writable for AddPartitionsToTxnRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.v3_and_below_transactional_id.write_ext(output, "self.v3_and_below_transactional_id", true)?;
        self.v3_and_below_producer_id.write(output)?;
        self.v3_and_below_producer_epoch.write(output)?;
        write_array(output, "self.v3_and_below_topics", &self.v3_and_below_topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AddPartitionsToTxnTopic, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AddPartitionsToTxnTopic {
    /// The name of the topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The partition indexes to add to the transaction
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AddPartitionsToTxnTopic {
    fn default() -> Self {
        AddPartitionsToTxnTopic {
            name: String::from(""),
            partitions: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AddPartitionsToTxnTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<i32>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_add_partitions_to_txn_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AddPartitionsToTxnTopic::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, AddPartitionsToTxnTopic::default());
    }
}

impl Readable for AddPartitionsToTxnTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = read_array::<i32>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AddPartitionsToTxnTopic {
            name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for AddPartitionsToTxnTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
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
        crate::test_utils::test_java_default::<AddPartitionsToTxnRequest>("AddPartitionsToTxnRequest", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AddPartitionsToTxnRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AddPartitionsToTxnRequest) {
            crate::test_utils::test_java_arbitrary(&data, "AddPartitionsToTxnRequest", 3);
        }
    }
}
